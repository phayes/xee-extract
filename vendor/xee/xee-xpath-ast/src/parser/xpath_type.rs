use chumsky::{input::ValueInput, prelude::*};
use xee_schema_type::Xs;
use xee_xpath_lexer::Token;
use xot::xmlname::NameStrInfo;

use crate::ast;
use crate::ast::Span;
use crate::error::ParserError;

use super::types::BoxedParser;

#[derive(Clone)]
pub(crate) struct ParserTypeOutput<'a, I>
where
    I: ValueInput<'a, Token = Token<'a>, Span = Span>,
{
    pub(crate) sequence_type: BoxedParser<'a, I, ast::SequenceType>,
    pub(crate) item_type: BoxedParser<'a, I, ast::ItemType>,
    pub(crate) single_type: BoxedParser<'a, I, ast::SingleType>,
}

pub(crate) fn parser_type<'a, I>(
    eqname: BoxedParser<'a, I, ast::NameS>,
    empty_call: BoxedParser<'a, I, Token<'a>>,
    kind_test: BoxedParser<'a, I, ast::KindTest>,
) -> ParserTypeOutput<'a, I>
where
    I: ValueInput<'a, Token = Token<'a>, Span = Span>,
{
    let single_type = eqname
        .clone()
        .then(just(Token::QuestionMark).or_not())
        .map(|(name, question_mark)| ast::SingleType {
            name,
            optional: question_mark.is_some(),
        })
        .boxed();

    let empty = just(Token::EmptySequence)
        .ignore_then(empty_call.clone())
        .to(ast::SequenceType::Empty)
        .boxed();

    let item_type_kind_test = kind_test.clone().map(ast::ItemType::KindTest);
    let item_type_empty = just(Token::Item)
        .ignore_then(empty_call.clone())
        .to(ast::ItemType::Item)
        .boxed();
    let item_type_atomic_or_union = eqname.clone().try_map(|name, _span| {
        Ok(ast::ItemType::AtomicOrUnionType(
            name_to_xs(&name.value).map_err(|_| ParserError::UnknownType {
                name: name.value.clone(),
                span: name.span,
            })?,
        ))
    });

    let any_function_test = just(Token::Function)
        .ignore_then(
            just(Token::Asterisk).delimited_by(just(Token::LeftParen), just(Token::RightParen)),
        )
        .to(ast::FunctionTest::AnyFunctionTest)
        .boxed();

    // TODO: ugly way to get item type out of recursive
    let mut item_type_ = None;

    let sequence_type = recursive(|sequence_type| {
        let typed_map_test_entry = (eqname
            .then_ignore(just(Token::Comma))
            .then(sequence_type.clone()))
        .try_map(|(key_type, value_type), _span| {
            Ok(ast::MapTest::TypedMapTest(Box::new(ast::TypedMapTest {
                key_type: name_to_xs(&key_type.value).map_err(|_| ParserError::UnknownType {
                    name: key_type.value.clone(),
                    span: key_type.span,
                })?,
                value_type,
            })))
        })
        .boxed();

        let typed_map_test = just(Token::Map)
            .ignore_then(
                typed_map_test_entry.delimited_by(just(Token::LeftParen), just(Token::RightParen)),
            )
            .boxed();

        let any_map_test = just(Token::Map)
            .ignore_then(
                just(Token::Asterisk).delimited_by(just(Token::LeftParen), just(Token::RightParen)),
            )
            .to(ast::MapTest::AnyMapTest)
            .boxed();

        let item_type_map_test = any_map_test
            .or(typed_map_test)
            .map(ast::ItemType::MapTest)
            .boxed();

        let typed_array_test = just(Token::Array)
            .ignore_then(
                sequence_type
                    .clone()
                    .delimited_by(just(Token::LeftParen), just(Token::RightParen))
                    .map(|item_type| {
                        ast::ArrayTest::TypedArrayTest(Box::new(ast::TypedArrayTest { item_type }))
                    }),
            )
            .boxed();

        let any_array_test = just(Token::Array)
            .ignore_then(
                just(Token::Asterisk).delimited_by(just(Token::LeftParen), just(Token::RightParen)),
            )
            .to(ast::ArrayTest::AnyArrayTest)
            .boxed();

        let item_type_array_test = any_array_test
            .or(typed_array_test)
            .map(ast::ItemType::ArrayTest)
            .boxed();

        let typed_function_param_list = sequence_type
            .clone()
            .separated_by(just(Token::Comma))
            .collect::<Vec<_>>()
            .boxed();

        let item_type = recursive(|item_type| {
            let typed_function_test = just(Token::Function)
                .ignore_then(
                    typed_function_param_list
                        .delimited_by(just(Token::LeftParen), just(Token::RightParen))
                        .then_ignore(just(Token::As))
                        .then(sequence_type)
                        .map(|(parameter_types, return_type)| {
                            ast::FunctionTest::TypedFunctionTest(Box::new(ast::TypedFunctionTest {
                                parameter_types,
                                return_type,
                            }))
                        }),
                )
                .boxed();
            let item_type_function_test = typed_function_test
                .or(any_function_test)
                .map(ast::ItemType::FunctionTest);

            let parenthesized_item_type =
                item_type.delimited_by(just(Token::LeftParen), just(Token::RightParen));
            choice((
                item_type_empty,
                item_type_array_test,
                item_type_map_test,
                item_type_function_test,
                item_type_kind_test,
                item_type_atomic_or_union,
                parenthesized_item_type,
            ))
        })
        .boxed();

        item_type_ = Some(item_type.clone());

        let occurrence = one_of([Token::QuestionMark, Token::Asterisk, Token::Plus])
            .map(|c| match c {
                Token::QuestionMark => ast::Occurrence::Option,
                Token::Asterisk => ast::Occurrence::Many,
                Token::Plus => ast::Occurrence::NonEmpty,
                _ => unreachable!(),
            })
            .or_not()
            .map(|o| o.unwrap_or(ast::Occurrence::One))
            .boxed();

        let item = item_type
            .clone()
            .then(occurrence)
            .map(|(item_type, occurrence)| ast::Item {
                item_type,
                occurrence,
            })
            .boxed();

        empty.or(item.map(ast::SequenceType::Item)).boxed()
    })
    .boxed();

    ParserTypeOutput {
        sequence_type,
        single_type,
        item_type: item_type_.unwrap(),
    }
}

pub(crate) fn name_to_xs(name: &ast::Name) -> Result<Xs, ()> {
    Xs::by_name(name.namespace(), name.local_name()).ok_or(())
}

// impl TryFrom<ast::Name> for Xs {
//     type Error = ();

//     fn try_from(name: ast::Name) -> Result<Xs, ()> {
//         Xs::by_name(name.namespace(), name.local_name()).ok_or(())
//     }
// }
