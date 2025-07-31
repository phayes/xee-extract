use chumsky::{input::ValueInput, prelude::*};
use std::borrow::Cow;
use xee_xpath_lexer::Token;

use crate::ast::Span;
use crate::{ast, error::ParserError};

use super::types::BoxedParser;
use super::xpath_type::name_to_xs;

#[derive(Clone)]
pub(crate) struct ParserKindTestOutput<'a, I>
where
    I: ValueInput<'a, Token = Token<'a>, Span = Span>,
{
    pub(crate) kind_test: BoxedParser<'a, I, ast::KindTest>,
}

pub(crate) fn parser_kind_test<'a, I>(
    eqname: BoxedParser<'a, I, ast::NameS>,
    empty_call: BoxedParser<'a, I, Token<'a>>,
    ncname: BoxedParser<'a, I, &'a str>,
    string: BoxedParser<'a, I, Cow<'a, str>>,
) -> ParserKindTestOutput<'a, I>
where
    I: ValueInput<'a, Token = Token<'a>, Span = Span>,
{
    let element_declaration = eqname.clone();
    let schema_element_test = just(Token::SchemaElement)
        .ignore_then(
            element_declaration.delimited_by(just(Token::LeftParen), just(Token::RightParen)),
        )
        .map(|name| ast::SchemaElementTest { name: name.value })
        .boxed();

    let name_or_wildcard = just(Token::Asterisk)
        .to(ast::NameOrWildcard::Wildcard)
        .or(eqname.clone().map_with(|name, extra| {
            // use default element namespace; we can do this without worrying
            // about context, as it's an element name test
            ast::NameOrWildcard::Name(
                name.value
                    .with_default_namespace(&extra.state().namespaces.default_element_namespace),
            )
        }))
        .boxed();

    let type_name = eqname.clone();

    let element_type_name = type_name
        .clone()
        .then(just(Token::QuestionMark).or_not())
        .try_map(|(name, question_mark), _span| {
            Ok(ast::TypeName {
                name: name_to_xs(&name.value).map_err(|_| ParserError::UnknownType {
                    name: name.value.clone(),
                    span: name.span,
                })?,
                can_be_nilled: question_mark.is_some(),
            })
        })
        .boxed();

    let element_test_content = name_or_wildcard
        .clone()
        .then((just(Token::Comma).ignore_then(element_type_name)).or_not())
        .map(
            |(name_or_wildcard, type_name)| ast::ElementOrAttributeTest {
                name_or_wildcard,
                type_name,
            },
        )
        .boxed();

    let element_test = just(Token::Element)
        .ignore_then(
            element_test_content
                .or_not()
                .delimited_by(just(Token::LeftParen), just(Token::RightParen)),
        )
        .boxed();

    let document_test_content = element_test
        .clone()
        .map(ast::DocumentTest::Element)
        .or(schema_element_test
            .clone()
            .map(ast::DocumentTest::SchemaElement))
        .boxed();

    let document_test = just(Token::DocumentNode)
        .ignore_then(
            document_test_content
                .or_not()
                .delimited_by(just(Token::LeftParen), just(Token::RightParen)),
        )
        .boxed();

    let attribute_test_content = name_or_wildcard
        .then((just(Token::Comma).ignore_then(type_name)).or_not())
        .try_map(|(name_or_wildcard, type_name), _span| {
            Ok(ast::ElementOrAttributeTest {
                name_or_wildcard,
                type_name: type_name
                    .map(|name| {
                        Ok(ast::TypeName {
                            name: name_to_xs(&name.value).map_err(|_| {
                                ParserError::UnknownType {
                                    name: name.value.clone(),
                                    span: name.span,
                                }
                            })?,
                            // this is not relevant for attributes
                            can_be_nilled: true,
                        })
                    })
                    .transpose()?,
            })
        })
        .boxed();

    let attribute_test = just(Token::Attribute)
        .ignore_then(
            attribute_test_content
                .or_not()
                .delimited_by(just(Token::LeftParen), just(Token::RightParen)),
        )
        .boxed();

    let any_test = just(Token::Node)
        .ignore_then(empty_call.clone())
        .to(ast::KindTest::Any)
        .boxed();

    let attribute_name = eqname.clone();
    let attribute_declaration = attribute_name;
    let schema_attribute_test = just(Token::SchemaAttribute)
        .ignore_then(
            attribute_declaration.delimited_by(just(Token::LeftParen), just(Token::RightParen)),
        )
        .map(|name| ast::SchemaAttributeTest { name: name.value })
        .boxed();

    let pi_test_content = ncname
        .clone()
        .map(|s| ast::PITest::Name(s.to_string()))
        .or(string.map(|s| ast::PITest::StringLiteral(s.to_string())))
        .boxed();

    let pi_test = just(Token::ProcessingInstruction)
        .ignore_then(
            pi_test_content
                .or_not()
                .delimited_by(just(Token::LeftParen), just(Token::RightParen)),
        )
        .boxed();

    let text_test = just(Token::Text)
        .ignore_then(empty_call.clone())
        .to(ast::KindTest::Text);
    let comment_test = just(Token::Comment)
        .ignore_then(empty_call.clone())
        .to(ast::KindTest::Comment);
    let namespace_node_test = just(Token::NamespaceNode)
        .ignore_then(empty_call.clone())
        .to(ast::KindTest::NamespaceNode);

    let kind_test = document_test
        .map(ast::KindTest::Document)
        .or(element_test.map(ast::KindTest::Element))
        .or(attribute_test.map(ast::KindTest::Attribute))
        .or(schema_element_test.map(ast::KindTest::SchemaElement))
        .or(schema_attribute_test.map(ast::KindTest::SchemaAttribute))
        .or(pi_test.map(ast::KindTest::PI))
        .or(comment_test)
        .or(text_test)
        .or(namespace_node_test)
        .or(any_test)
        .boxed();

    ParserKindTestOutput { kind_test }
}
