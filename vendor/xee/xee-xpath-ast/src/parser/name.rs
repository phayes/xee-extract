use chumsky::inspector::SimpleState;
use chumsky::{input::ValueInput, prelude::*};

use xee_xpath_lexer::Token;

use crate::ast::Span;
use crate::span::WithSpan;
use crate::{ast, error::ParserError};

use super::types::{BoxedParser, State};

#[derive(Clone)]
pub(crate) struct ParserNameOutput<'a, I>
where
    I: ValueInput<'a, Token = Token<'a>, Span = Span>,
{
    pub(crate) eqname: BoxedParser<'a, I, ast::NameS>,
    pub(crate) ncname: BoxedParser<'a, I, &'a str>,
}

pub(crate) fn parser_name<'a, I>() -> ParserNameOutput<'a, I>
where
    I: ValueInput<'a, Token = Token<'a>, Span = Span>,
{
    let ncname = select! {
        Token::NCName(s) => s,

    }
    .boxed();

    let prefixed_qname_token = select! {
        Token::PrefixedQName(prefixed) => prefixed
    }
    .boxed();

    let uri_qualified_name_token = select! {
        Token::URIQualifiedName(name) => name
    }
    .boxed();

    let ncname = ncname.clone().or(parser_keyword()).boxed();

    let prefixed_name = prefixed_qname_token
        .try_map_with(|prefixed_qname, extra| {
            let state: &mut SimpleState<State> = extra.state();
            ast::Name::prefixed(
                prefixed_qname.prefix,
                prefixed_qname.local_name,
                move |prefix| state.namespaces.by_prefix(prefix).map(|s| s.to_string()),
            )
            .map(|name| name.with_span(extra.span()))
            .map_err(|_e| ParserError::UnknownPrefix {
                prefix: prefixed_qname.prefix.to_string(),
                span: extra.span(),
            })
        })
        .boxed();

    let qname = prefixed_name
        .or(ncname
            .clone()
            .map_with(|local_name, extra| ast::Name::name(local_name).with_span(extra.span())))
        .boxed();

    let uri_qualified_name = uri_qualified_name_token
        .map_with(|uri_qualified_name, extra| {
            ast::Name::namespaced(
                uri_qualified_name.local_name.to_string(),
                uri_qualified_name.uri.to_string(),
                |_| Some(String::new()),
            )
            .unwrap()
            .with_span(extra.span())
        })
        .boxed();

    let eqname = qname.or(uri_qualified_name).boxed();

    ParserNameOutput { eqname, ncname }
}

// this is required as even though some tokens are reserved
// as function names, they're *not* reserved as NCNames. So we
// make them honorary ncnames after all.
fn parser_keyword<'a, I>() -> BoxedParser<'a, I, &'a str>
where
    I: ValueInput<'a, Token = Token<'a>, Span = Span>,
{
    // this implementation seems unfortunate but I cannot find
    // a way to turn a logos token back into the original string
    choice::<_>([
        just(Token::Ancestor).to("ancestor"),
        just(Token::AncestorOrSelf).to("ancestor-or-self"),
        just(Token::And).to("and"),
        just(Token::Array).to("array"),
        just(Token::As).to("as"),
        just(Token::Attribute).to("attribute"),
        just(Token::Cast).to("cast"),
        just(Token::Castable).to("castable"),
        just(Token::Child).to("child"),
        just(Token::Comment).to("comment"),
        just(Token::Descendant).to("descendant"),
        just(Token::DescendantOrSelf).to("descendant-or-self"),
        just(Token::Div).to("div"),
        just(Token::DocumentNode).to("document-node"),
        just(Token::Element).to("element"),
        just(Token::Else).to("else"),
        just(Token::EmptySequence).to("empty-sequence"),
        just(Token::Eq).to("eq"),
        just(Token::Every).to("every"),
        just(Token::Except).to("except"),
        just(Token::Following).to("following"),
        just(Token::FollowingSibling).to("following-sibling"),
        just(Token::For).to("for"),
        just(Token::Function).to("function"),
        just(Token::Ge).to("ge"),
        just(Token::Gt).to("gt"),
        just(Token::Idiv).to("idiv"),
        just(Token::If).to("if"),
        just(Token::In).to("in"),
        just(Token::Instance).to("instance"),
        just(Token::Intersect).to("intersect"),
        just(Token::Is).to("is"),
        just(Token::Item).to("item"),
        just(Token::Le).to("le"),
        just(Token::Let).to("let"),
        just(Token::Lt).to("lt"),
        just(Token::Map).to("map"),
        just(Token::Mod).to("mod"),
        just(Token::Namespace).to("namespace"),
        just(Token::NamespaceNode).to("namespace-node"),
        just(Token::Ne).to("ne"),
        just(Token::Node).to("node"),
        just(Token::Of).to("of"),
        just(Token::Or).to("or"),
        just(Token::Parent).to("parent"),
        just(Token::Preceding).to("preceding"),
        just(Token::PrecedingSibling).to("preceding-sibling"),
        just(Token::ProcessingInstruction).to("processing-instruction"),
        just(Token::Return).to("return"),
        just(Token::Satisfies).to("satisfies"),
        just(Token::SchemaAttribute).to("schema-attribute"),
        just(Token::SchemaElement).to("schema-element"),
        just(Token::Self_).to("self"),
        just(Token::Some).to("some"),
        just(Token::Text).to("text"),
        just(Token::Then).to("then"),
        just(Token::To).to("to"),
        just(Token::Treat).to("treat"),
        just(Token::Union).to("union"),
        just(Token::Switch).to("switch"),
        just(Token::Typeswitch).to("typeswitch"),
    ])
    .boxed()
}
