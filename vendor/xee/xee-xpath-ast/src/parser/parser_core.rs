use chumsky::{input::ValueInput, prelude::*};
use std::iter::once;
use xee_xpath_lexer::Token;

use crate::ast;
use crate::ast::Span;

use crate::span::{Spanned, WithSpan};
use crate::FN_NAMESPACE;

use super::axis_node_test::{parser_axis_node_test, ParserAxisNodeTestOutput};
use super::kind_test::{parser_kind_test, ParserKindTestOutput};
use super::name::{parser_name, ParserNameOutput};
use super::primary::{check_reserved, parser_primary, ParserPrimaryOutput};
use super::signature::{parser_signature, ParserSignatureOutput};
use super::types::BoxedParser;
use super::xpath_type::{parser_type, ParserTypeOutput};

#[derive(Clone)]
pub(crate) struct ParserOutput<'a, I>
where
    I: ValueInput<'a, Token = Token<'a>, Span = Span>,
{
    pub(crate) name: BoxedParser<'a, I, ast::NameS>,
    pub(crate) expr_single: BoxedParser<'a, I, ast::ExprSingleS>,
    pub(crate) expr_single_core: BoxedParser<'a, I, ast::ExprSingleS>,
    pub(crate) signature: BoxedParser<'a, I, ast::Signature>,
    pub(crate) item_type: BoxedParser<'a, I, ast::ItemType>,
    pub(crate) sequence_type: BoxedParser<'a, I, ast::SequenceType>,
    pub(crate) kind_test: BoxedParser<'a, I, ast::KindTest>,
    pub(crate) xpath: BoxedParser<'a, I, ast::XPath>,
    pub(crate) xpath_right_brace: BoxedParser<'a, I, ast::XPath>,
}

pub(crate) fn parser<'a, I>() -> ParserOutput<'a, I>
where
    I: ValueInput<'a, Token = Token<'a>, Span = Span>,
{
    let ParserNameOutput { eqname, ncname } = parser_name();

    let ParserPrimaryOutput {
        literal,
        var_ref,
        context_item_expr,
        named_function_ref,
        string,
    } = parser_primary(eqname.clone());

    let empty_call = just(Token::LeftParen)
        .ignore_then(just(Token::RightParen))
        .boxed();

    let ParserKindTestOutput { kind_test } = parser_kind_test(
        eqname.clone(),
        empty_call.clone(),
        ncname.clone(),
        string.clone(),
    );

    let ParserTypeOutput {
        sequence_type,
        item_type,
        single_type,
    } = parser_type(eqname.clone(), empty_call.clone(), kind_test.clone());

    let ParserAxisNodeTestOutput { axis_node_test, .. } =
        parser_axis_node_test(eqname.clone(), kind_test.clone());

    let ParserSignatureOutput {
        signature,
        param_list,
    } = parser_signature(eqname.clone(), sequence_type.clone());

    // TODO: ugly way to get expr out of recursive
    let mut expr_ = None;

    let expr_single = recursive(|expr_single| {
        let expr = expr_single
            .clone()
            .separated_by(just(Token::Comma))
            .at_least(1)
            .collect::<Vec<_>>()
            .map_with(|exprs, extra| ast::Expr(exprs).with_span(extra.span()))
            .boxed();

        expr_ = Some(expr.clone());

        // unlike a normal expr, this can create an empty expression sequence,
        // which is used to represent to represent an empty sequence
        let parenthesized_expr = expr
            .clone()
            .or_not()
            .delimited_by(just(Token::LeftParen), just(Token::RightParen))
            .map_with(|expr, extra| expr.map(|expr| expr.value).with_span(extra.span()))
            .boxed();

        let parenthesized_expr_primary = parenthesized_expr
            .clone()
            .map_with(|expr, extra| ast::PrimaryExpr::Expr(expr).with_span(extra.span()))
            .boxed();

        let argument_placeholder = just(Token::QuestionMark)
            .map(|_| ArgumentOrPlaceholder::Placeholder)
            .boxed();
        let argument = expr_single
            .clone()
            .map(ArgumentOrPlaceholder::Argument)
            .or(argument_placeholder)
            .boxed();
        let argument_list = argument
            .separated_by(just(Token::Comma))
            .collect::<Vec<_>>()
            .delimited_by(just(Token::LeftParen), just(Token::RightParen))
            .boxed();

        enum PostfixOrPlaceholderWrapper {
            Postfix(ast::Postfix),
            PlaceholderWrapper(Vec<ast::ExprSingleS>, Vec<ast::Param>, Span),
        }

        let predicate = expr
            .clone()
            .delimited_by(just(Token::LeftBracket), just(Token::RightBracket))
            .map(ast::Postfix::Predicate)
            .map(PostfixOrPlaceholderWrapper::Postfix)
            .boxed();

        let argument_list_postfix = argument_list
            .clone()
            .map_with(|arguments, extra| {
                let (arguments, params) = placeholder_arguments(&arguments);
                if params.is_empty() {
                    PostfixOrPlaceholderWrapper::Postfix(ast::Postfix::ArgumentList(arguments))
                } else {
                    PostfixOrPlaceholderWrapper::PlaceholderWrapper(arguments, params, extra.span())
                }
            })
            .boxed();

        let integer = select! {
            Token::IntegerLiteral(i) => i,
        };

        let key_specifier = ncname
            .map(|name| ast::KeySpecifier::NcName(name.to_string()))
            .or(integer.map(ast::KeySpecifier::Integer))
            .or(parenthesized_expr.clone().map(ast::KeySpecifier::Expr))
            .or(just(Token::Asterisk).to(ast::KeySpecifier::Star));

        let lookup = just(Token::QuestionMark)
            .ignore_then(key_specifier.clone())
            .map(ast::Postfix::Lookup)
            .map(PostfixOrPlaceholderWrapper::Postfix)
            .boxed();

        let postfix = predicate.or(argument_list_postfix).or(lookup).boxed();

        fn static_function_call(
            name: ast::NameS,
            arguments: Vec<ArgumentOrPlaceholder>,
            default_function_namespace: &str,
            span: Span,
        ) -> ast::PrimaryExprS {
            let name = name.map(|name| name.with_default_namespace(default_function_namespace));
            let (arguments, params) = placeholder_arguments(&arguments);
            if params.is_empty() {
                ast::PrimaryExpr::FunctionCall(ast::FunctionCall { name, arguments })
                    .with_span(span)
            } else {
                let inner_function_call =
                    ast::PrimaryExpr::FunctionCall(ast::FunctionCall { name, arguments })
                        .with_empty_span();
                let step_expr = ast::StepExpr::PrimaryExpr(inner_function_call).with_empty_span();
                placeholder_wrapper_function(step_expr, params, span)
            }
        }

        let function_call = eqname
            .clone()
            .then(argument_list.clone())
            .try_map_with(move |(name, arguments), extra| {
                let span = extra.span();
                check_reserved(&name, span)?;
                Ok(static_function_call(
                    name,
                    arguments,
                    &extra.state().namespaces.default_function_namespace,
                    span,
                ))
            })
            .boxed();

        let enclosed_expr = (expr.clone().or_not())
            .delimited_by(just(Token::LeftBrace), just(Token::RightBrace))
            .boxed();

        let function_body = enclosed_expr
            .clone()
            .map_with(|expr, extra| {
                if let Some(expr) = expr {
                    Some(expr.value).with_span(extra.span())
                } else {
                    None.with_span(extra.span())
                }
            })
            .boxed();

        let inline_function_expr = just(Token::Function)
            .ignore_then(param_list.delimited_by(just(Token::LeftParen), just(Token::RightParen)))
            .then(just(Token::As).ignore_then(sequence_type.clone()).or_not())
            .then(function_body)
            .map_with(|((params, return_type), body), extra| {
                ast::PrimaryExpr::InlineFunction(ast::InlineFunction {
                    params,
                    return_type,
                    body,
                    wrapper: false,
                })
                .with_span(extra.span())
            })
            .boxed();

        let map_constructor_entry = expr_single
            .clone()
            .then_ignore(just(Token::Colon))
            .then(expr_single.clone())
            .map(|(key, value)| ast::MapConstructorEntry { key, value })
            .boxed();

        let map_contents = map_constructor_entry
            .clone()
            .separated_by(just(Token::Comma))
            .collect::<Vec<_>>()
            .boxed()
            .delimited_by(just(Token::LeftBrace), just(Token::RightBrace))
            .boxed();

        let map_constructor = just(Token::Map)
            .ignore_then(map_contents)
            .map_with(|entries, extra| {
                ast::PrimaryExpr::MapConstructor(ast::MapConstructor { entries })
                    .with_span(extra.span())
            })
            .boxed();

        let curly_array_constructor = just(Token::Array)
            .ignore_then(enclosed_expr)
            .map_with(|expr, extra| {
                ast::ArrayConstructor::Curly(expr.map(|expr| expr.value).with_span(extra.span()))
            })
            .boxed();
        let square_array_constructor = expr_single
            .clone()
            .separated_by(just(Token::Comma))
            .collect::<Vec<_>>()
            .map_with(|exprs, extra| ast::Expr(exprs).with_span(extra.span()))
            .delimited_by(just(Token::LeftBracket), just(Token::RightBracket))
            .map(ast::ArrayConstructor::Square)
            .boxed();
        let array_constructor = square_array_constructor
            .or(curly_array_constructor)
            .boxed()
            .map_with(|constructor, extra| {
                ast::PrimaryExpr::ArrayConstructor(constructor).with_span(extra.span())
            });

        let unary_lookup = just(Token::QuestionMark)
            .ignore_then(key_specifier)
            .boxed()
            .map_with(|key_specifier, extra| {
                ast::PrimaryExpr::UnaryLookup(key_specifier).with_span(extra.span())
            });

        let primary_expr = parenthesized_expr_primary
            .or(literal)
            .or(var_ref.clone())
            .or(context_item_expr)
            .or(named_function_ref)
            .or(inline_function_expr)
            .or(function_call)
            .or(map_constructor)
            .or(array_constructor)
            .or(unary_lookup)
            .boxed();

        let postfix_expr = primary_expr
            .then(postfix.repeated().collect::<Vec<_>>())
            .map_with(|(primary, postfixes), extra| {
                // in case of a placeholder argument list we need to
                // wrap the existing primary
                let mut normal_postfixes = Vec::new();
                let mut primary = primary;
                for postfix in postfixes {
                    match postfix {
                        PostfixOrPlaceholderWrapper::Postfix(postfix) => {
                            normal_postfixes.push(postfix)
                        }
                        PostfixOrPlaceholderWrapper::PlaceholderWrapper(
                            arguments,
                            params,
                            span,
                        ) => {
                            normal_postfixes.push(ast::Postfix::ArgumentList(arguments));
                            let step_expr = ast::StepExpr::PostfixExpr {
                                primary,
                                postfixes: normal_postfixes.clone(),
                            }
                            .with_empty_span();
                            // replace primary with a placeholder wrapper function
                            primary = placeholder_wrapper_function(step_expr, params, span);
                            // now collect more postfixes
                            normal_postfixes.clear();
                        }
                    }
                }
                if normal_postfixes.is_empty() {
                    ast::StepExpr::PrimaryExpr(primary).with_span(extra.span())
                } else {
                    ast::StepExpr::PostfixExpr {
                        primary,
                        postfixes: normal_postfixes,
                    }
                    .with_span(extra.span())
                }
            })
            .boxed();

        let predicate = expr
            .clone()
            .delimited_by(just(Token::LeftBracket), just(Token::RightBracket))
            .boxed();

        let predicate_list = predicate.repeated().collect::<Vec<_>>().boxed();

        let axis_step = axis_node_test
            .then(predicate_list)
            .map_with(|((axis, node_test), predicates), extra| {
                ast::StepExpr::AxisStep(ast::AxisStep {
                    axis,
                    node_test,
                    predicates,
                })
                .with_span(extra.span())
            })
            .boxed();

        let step_expr = postfix_expr.or(axis_step).boxed();

        let relative_path_expr = step_expr
            .clone()
            .then(
                just(Token::Slash)
                    .or(just(Token::DoubleSlash))
                    .then(step_expr.clone())
                    .repeated()
                    .collect::<Vec<_>>(),
            )
            .map(|(first_step, rest_steps)| {
                let mut steps = vec![first_step];
                for (token, step) in rest_steps {
                    match token {
                        Token::Slash => {}
                        Token::DoubleSlash => {
                            steps.push(
                                ast::StepExpr::AxisStep(ast::AxisStep {
                                    axis: ast::Axis::DescendantOrSelf,
                                    node_test: ast::NodeTest::KindTest(ast::KindTest::Any),
                                    predicates: vec![],
                                })
                                .with_empty_span(),
                            );
                        }
                        _ => unreachable!(),
                    }
                    steps.push(step);
                }
                steps
            })
            .boxed();

        let slash_prefix_path_expr = just(Token::Slash)
            .to_span()
            .then(relative_path_expr.clone().or_not())
            .map(|(slash_span, steps)| {
                let root_step = root_step(slash_span);
                if let Some(steps) = steps {
                    let all_steps = once(root_step).chain(steps).collect();
                    ast::PathExpr { steps: all_steps }
                } else {
                    ast::PathExpr {
                        steps: vec![root_step],
                    }
                }
            })
            .boxed();

        let doubleslash_prefix_path_expr = just(Token::DoubleSlash)
            .to_span()
            .then(relative_path_expr.clone().or_not())
            .map(|(double_slash_span, steps)| {
                let root_step = root_step(double_slash_span);
                let descendant_step = ast::StepExpr::AxisStep(ast::AxisStep {
                    axis: ast::Axis::DescendantOrSelf,
                    node_test: ast::NodeTest::KindTest(ast::KindTest::Any),
                    predicates: vec![],
                })
                .with_span(double_slash_span);
                if let Some(steps) = steps {
                    let all_steps = once(root_step)
                        .chain(once(descendant_step).chain(steps))
                        .collect();
                    ast::PathExpr { steps: all_steps }
                } else {
                    ast::PathExpr {
                        steps: vec![root_step, descendant_step],
                    }
                }
            })
            .boxed();

        let path_expr = doubleslash_prefix_path_expr
            .or(slash_prefix_path_expr)
            .or(relative_path_expr.map(|steps| ast::PathExpr { steps }))
            .boxed();

        let value_expr = path_expr
            .clone()
            .separated_by(just(Token::ExclamationMark))
            .at_least(1)
            .collect::<Vec<_>>()
            .map_with(|path_exprs, extra| {
                if path_exprs.len() == 1 {
                    ast::ExprSingle::Path(path_exprs[0].clone()).with_span(extra.span())
                } else {
                    ast::ExprSingle::Apply(ast::ApplyExpr {
                        operator: ast::ApplyOperator::SimpleMap(path_exprs[1..].to_vec()),
                        path_expr: path_exprs[0].clone(),
                    })
                    .with_span(extra.span())
                }
            })
            .boxed();

        let unary_operator = just(Token::Minus)
            .to(ast::UnaryOperator::Minus)
            .or(just(Token::Plus).to(ast::UnaryOperator::Plus))
            .boxed();

        let unary_expr = unary_operator
            .repeated()
            .collect::<Vec<_>>()
            .then(value_expr.clone())
            .map_with(|(unary_operators, expr), extra| {
                if unary_operators.is_empty() {
                    expr
                } else {
                    ast::ExprSingle::Apply(ast::ApplyExpr {
                        operator: ast::ApplyOperator::Unary(unary_operators),
                        path_expr: expr_single_to_path_expr(expr),
                    })
                    .with_span(extra.span())
                }
            })
            .boxed();

        enum ArrowFunctionSpecifier {
            EQName(ast::NameS),
            VarRef(ast::PrimaryExprS),
            ParenthesizedExpr(Spanned<Option<ast::Expr>>),
        }
        let arrow_function_specifier = (eqname.clone().map(ArrowFunctionSpecifier::EQName))
            .or(var_ref.clone().map(ArrowFunctionSpecifier::VarRef))
            .or(parenthesized_expr
                .clone()
                .map(ArrowFunctionSpecifier::ParenthesizedExpr));

        fn dynamic_function_call(
            primary: ast::PrimaryExprS,
            argument_list: Vec<ArgumentOrPlaceholder>,
            span: Span,
        ) -> ast::ExprSingleS {
            let (arguments, params) = placeholder_arguments(&argument_list);
            let primary = if params.is_empty() {
                primary
            } else {
                let step_expr = ast::StepExpr::PrimaryExpr(primary).with_span(span);
                placeholder_wrapper_function(step_expr, params, span)
            };

            ast::ExprSingle::Path(ast::PathExpr {
                steps: vec![ast::StepExpr::PostfixExpr {
                    primary,
                    postfixes: vec![ast::Postfix::ArgumentList(arguments)],
                }
                .with_span(span)],
            })
            .with_span(span)
        }

        let arrow_expr = unary_expr
            .then(
                (just(Token::Arrow)
                    .ignore_then(arrow_function_specifier)
                    .then(argument_list.clone()))
                .repeated()
                .collect::<Vec<(ArrowFunctionSpecifier, Vec<ArgumentOrPlaceholder>)>>(),
            )
            .map_with(|(unary_expr, arrow_function_specifiers), extra| {
                if arrow_function_specifiers.is_empty() {
                    return unary_expr;
                }
                arrow_function_specifiers.into_iter().fold(
                    unary_expr,
                    |expr, (specifier, argument_list)| {
                        let mut argument_list = argument_list.clone();
                        argument_list.insert(0, ArgumentOrPlaceholder::Argument(expr));

                        match specifier {
                            ArrowFunctionSpecifier::EQName(name) => {
                                let span = extra.span();
                                primary_expr_to_expr_single(static_function_call(
                                    name.clone(),
                                    argument_list,
                                    &extra.state().namespaces.default_function_namespace,
                                    span,
                                ))
                            }
                            ArrowFunctionSpecifier::VarRef(primary) => {
                                dynamic_function_call(primary, argument_list, extra.span())
                            }
                            ArrowFunctionSpecifier::ParenthesizedExpr(parenthesized_expr) => {
                                let primary = ast::PrimaryExpr::Expr(parenthesized_expr)
                                    .with_span(extra.span());
                                dynamic_function_call(primary, argument_list, extra.span())
                            }
                        }
                    },
                )
            });

        let cast_expr = arrow_expr
            .then(
                just(Token::Cast)
                    .ignore_then(just(Token::As))
                    .ignore_then(single_type.clone())
                    .or_not(),
            )
            .map_with(|(expr, single_type), extra| {
                if let Some(single_type) = single_type {
                    ast::ExprSingle::Apply(ast::ApplyExpr {
                        path_expr: expr_single_to_path_expr(expr),
                        operator: ast::ApplyOperator::Cast(single_type),
                    })
                    .with_span(extra.span())
                } else {
                    expr
                }
            })
            .boxed();

        let castable_expr = cast_expr
            .then(
                just(Token::Castable)
                    .ignore_then(just(Token::As))
                    .ignore_then(single_type)
                    .or_not(),
            )
            .map_with(|(expr, single_type), extra| {
                if let Some(single_type) = single_type {
                    ast::ExprSingle::Apply(ast::ApplyExpr {
                        path_expr: expr_single_to_path_expr(expr),
                        operator: ast::ApplyOperator::Castable(single_type),
                    })
                    .with_span(extra.span())
                } else {
                    expr
                }
            })
            .boxed();

        let treat_expr = castable_expr
            .then(
                just(Token::Treat)
                    .ignore_then(just(Token::As))
                    .ignore_then(sequence_type.clone())
                    .or_not(),
            )
            .map_with(|(expr, sequence_type), extra| {
                if let Some(sequence_type) = sequence_type {
                    ast::ExprSingle::Apply(ast::ApplyExpr {
                        path_expr: expr_single_to_path_expr(expr),
                        operator: ast::ApplyOperator::Treat(sequence_type),
                    })
                    .with_span(extra.span())
                } else {
                    expr
                }
            })
            .boxed();

        let instance_of_expr = treat_expr
            .then(
                just(Token::Instance)
                    .ignore_then(just(Token::Of))
                    .ignore_then(sequence_type.clone())
                    .or_not(),
            )
            .map_with(|(expr, sequence_type), extra| {
                if let Some(sequence_type) = sequence_type {
                    ast::ExprSingle::Apply(ast::ApplyExpr {
                        path_expr: expr_single_to_path_expr(expr),
                        operator: ast::ApplyOperator::InstanceOf(sequence_type),
                    })
                    .with_span(extra.span())
                } else {
                    expr
                }
            })
            .boxed();

        let intersect_except_operator = just(Token::Intersect)
            .to(ast::BinaryOperator::Intersect)
            .or(just(Token::Except).to(ast::BinaryOperator::Except))
            .boxed();

        let intersect_except_expr =
            binary_expr_op(instance_of_expr, intersect_except_operator).boxed();

        let union_operator = just(Token::Pipe)
            .map(|_| ast::BinaryOperator::Union)
            .or(just(Token::Union).map(|_| ast::BinaryOperator::Union))
            .boxed();

        let union_expr = binary_expr_op(intersect_except_expr, union_operator).boxed();

        let multiplicative_operator = choice::<_>([
            just(Token::Asterisk).to(ast::BinaryOperator::Mul),
            just(Token::Div).to(ast::BinaryOperator::Div),
            just(Token::Idiv).to(ast::BinaryOperator::IntDiv),
            just(Token::Mod).to(ast::BinaryOperator::Mod),
        ])
        .boxed();

        let multiplicative_expr = binary_expr_op(union_expr, multiplicative_operator).boxed();

        let additive_operator = one_of([Token::Plus, Token::Minus])
            .map(|c| match c {
                Token::Plus => ast::BinaryOperator::Add,
                Token::Minus => ast::BinaryOperator::Sub,
                _ => unreachable!(),
            })
            .boxed();
        let additive_expr = binary_expr_op(multiplicative_expr, additive_operator).boxed();

        let range_expr = binary_expr(additive_expr, Token::To, ast::BinaryOperator::Range).boxed();
        let string_concat_expr =
            binary_expr(range_expr, Token::DoublePipe, ast::BinaryOperator::Concat).boxed();
        use ast::BinaryOperator::*;

        let comparison_operator = choice::<_>([
            just(Token::Equal).to(GenEq),
            just(Token::NotEqual).to(GenNe),
            just(Token::LessThan).to(GenLt),
            just(Token::LessThanEqual).to(GenLe),
            just(Token::GreaterThan).to(GenGt),
            just(Token::GreaterThanEqual).to(GenGe),
            just(Token::Eq).to(ValueEq),
            just(Token::Ne).to(ValueNe),
            just(Token::Lt).to(ValueLt),
            just(Token::Le).to(ValueLe),
            just(Token::Gt).to(ValueGt),
            just(Token::Ge).to(ValueGe),
            just(Token::Is).to(Is),
            just(Token::Precedes).to(Precedes),
            just(Token::Follows).to(Follows),
        ])
        .boxed();

        // unlike other binary operators, a comparison expression may only
        // contain a single comparison operator (unless parens are used)
        let comparison_expr = (string_concat_expr
            .clone()
            .then(comparison_operator)
            .then(string_concat_expr.clone())
            .map_with(|((left, operator), right), extra| {
                ast::ExprSingle::Binary(ast::BinaryExpr {
                    operator,
                    left: expr_single_to_path_expr(left),
                    right: expr_single_to_path_expr(right),
                })
                .with_span(extra.span())
            }))
        .or(string_concat_expr.map_with(|expr, extra| {
            ast::ExprSingle::Path(expr_single_to_path_expr(expr)).with_span(extra.span())
        }))
        .boxed();

        let and_expr = binary_expr(comparison_expr, Token::And, ast::BinaryOperator::And).boxed();
        let or_expr = binary_expr(and_expr, Token::Or, ast::BinaryOperator::Or).boxed();

        let path_expr = or_expr
            .map_with(|expr_single, extra| {
                ast::ExprSingle::Path(expr_single_to_path_expr(expr_single)).with_span(extra.span())
            })
            .boxed();

        let simple_let_binding = just(Token::Dollar)
            .ignore_then(eqname.clone())
            .then_ignore(just(Token::ColonEqual))
            .then(expr_single.clone())
            .boxed();

        let simple_let_clause = just(Token::Let)
            .ignore_then(
                simple_let_binding
                    .clone()
                    .separated_by(just(Token::Comma))
                    .at_least(1)
                    .collect::<Vec<_>>(),
            )
            .boxed();

        let let_expr = simple_let_clause
            .then_ignore(just(Token::Return))
            .then(expr_single.clone())
            .map_with(|(bindings, return_expr), extra| {
                bindings
                    .iter()
                    .rev()
                    .fold(return_expr, |return_expr, (var_name, var_expr)| {
                        ast::ExprSingle::Let(ast::LetExpr {
                            var_name: var_name.clone(),
                            var_expr: Box::new(var_expr.clone()),
                            return_expr: Box::new(return_expr),
                        })
                        .with_span(extra.span())
                    })
            })
            .boxed();

        let simple_for_binding = just(Token::Dollar)
            .ignore_then(eqname.clone())
            .then_ignore(just(Token::In))
            .then(expr_single.clone())
            .boxed();

        let for_bindings = simple_for_binding
            .clone()
            .separated_by(just(Token::Comma))
            .at_least(1)
            .collect::<Vec<_>>()
            .boxed();

        let simple_for_clause = just(Token::For).ignore_then(for_bindings.clone()).boxed();

        let for_expr = simple_for_clause
            .clone()
            .then_ignore(just(Token::Return))
            .then(expr_single.clone())
            .map_with(|(bindings, return_expr), extra| {
                bindings
                    .iter()
                    .rev()
                    .fold(return_expr, |return_expr, (var_name, var_expr)| {
                        ast::ExprSingle::For(ast::ForExpr {
                            var_name: var_name.clone(),
                            var_expr: Box::new(var_expr.clone()),
                            return_expr: Box::new(return_expr),
                        })
                        .with_span(extra.span())
                    })
            })
            .boxed();

        let if_expr = just(Token::If)
            .ignore_then(
                expr.delimited_by(just(Token::LeftParen), just(Token::RightParen))
                    .clone(),
            )
            .then_ignore(just(Token::Then))
            .then(expr_single.clone())
            .then_ignore(just(Token::Else))
            .then(expr_single.clone())
            .map_with(|((condition, then), else_), extra| {
                ast::ExprSingle::If(ast::IfExpr {
                    condition,
                    then: Box::new(then),
                    else_: Box::new(else_),
                })
                .with_span(extra.span())
            })
            .boxed();

        let quantified_expr = choice::<_>([
            just(Token::Some).to(ast::Quantifier::Some),
            just(Token::Every).to(ast::Quantifier::Every),
        ])
        .then(for_bindings.clone())
        .then_ignore(just(Token::Satisfies))
        .then(expr_single)
        .map_with(|((quantifier, bindings), satisfies_expr), extra| {
            bindings
                .iter()
                .rev()
                .fold(satisfies_expr, |satisfies_expr, (var_name, var_expr)| {
                    ast::ExprSingle::Quantified(ast::QuantifiedExpr {
                        quantifier: quantifier.clone(),
                        var_name: var_name.clone(),
                        var_expr: Box::new(var_expr.clone()),
                        satisfies_expr: Box::new(satisfies_expr),
                    })
                    .with_span(extra.span())
                })
        })
        .boxed();

        let expr_single_ = let_expr
            .or(for_expr)
            .or(if_expr)
            .or(quantified_expr)
            .or(path_expr)
            .boxed();

        expr_single_
    })
    .boxed();

    let name = eqname.clone().then_ignore(end()).boxed();
    let expr_single_core = expr_single.clone();
    let expr_single = expr_single.then_ignore(end()).boxed();
    let xpath = expr_
        .clone()
        .unwrap()
        .then_ignore(end())
        .map(ast::XPath)
        .boxed();
    // TODO: how well does this scale? what we intend is parse up to the end of
    // the right brace, and after that stop caring and tokenizing.
    // Unfortunately I can't seem to make it do that without actually going
    // through the rest of the tokens. But I'm afraid that `any().repeated()`
    // tries to tokenize the rest of the tet of the XSLT template in which this
    // xpath is embedded, which is not very efficient.
    let xpath_right_brace = expr_
        .unwrap()
        .then_ignore(just(Token::RightBrace))
        .then_ignore(any().repeated())
        .map(ast::XPath)
        .boxed();
    let signature = signature.then_ignore(end()).boxed();
    let sequence_type = sequence_type.then_ignore(end()).boxed();
    let item_type = item_type.then_ignore(end()).boxed();
    let kind_test = kind_test.then_ignore(end()).boxed();

    ParserOutput {
        name,
        expr_single,
        expr_single_core,
        xpath,
        xpath_right_brace,
        signature,
        sequence_type,
        item_type,
        kind_test,
    }
}

fn binary_expr<'a, I>(
    sub_expr: BoxedParser<'a, I, ast::ExprSingleS>,
    operator_token: Token<'a>,
    operator: ast::BinaryOperator,
) -> BoxedParser<'a, I, ast::ExprSingleS>
where
    I: Input<'a, Token = Token<'a>, Span = Span> + ValueInput<'a>,
{
    binary_expr_op(
        sub_expr,
        just(operator_token).map(move |_| operator).boxed(),
    )
}

fn binary_expr_op<'a, I>(
    sub_expr: BoxedParser<'a, I, ast::ExprSingleS>,
    operator: BoxedParser<'a, I, ast::BinaryOperator>,
) -> BoxedParser<'a, I, ast::ExprSingleS>
where
    I: Input<'a, Token = Token<'a>, Span = Span> + ValueInput<'a>,
{
    sub_expr
        .clone()
        .foldl(
            operator.then(sub_expr).repeated(),
            move |left, (operator, right)| {
                let span: SimpleSpan = (left.span.start..right.span.end).into();
                ast::ExprSingle::Binary(ast::BinaryExpr {
                    operator,
                    left: expr_single_to_path_expr(left),
                    right: expr_single_to_path_expr(right),
                })
                .with_span(span)
            },
        )
        .boxed()
}

fn expr_single_to_path_expr(expr: ast::ExprSingleS) -> ast::PathExpr {
    let span = expr.span;
    match expr.value {
        ast::ExprSingle::Path(path) => path,
        _ => ast::PathExpr {
            steps: vec![ast::StepExpr::PrimaryExpr(
                ast::PrimaryExpr::Expr(Some(ast::Expr(vec![expr])).with_span(span)).with_span(span),
            )
            .with_span(span)],
        },
    }
}

fn primary_expr_to_expr_single(primary_expr: ast::PrimaryExprS) -> ast::ExprSingleS {
    let span = primary_expr.span;
    ast::ExprSingle::Path(ast::PathExpr {
        steps: vec![ast::StepExpr::PrimaryExpr(primary_expr).with_span(span)],
    })
    .with_span(span)
}

fn root_step(span: Span) -> ast::StepExprS {
    let path_arg = ast::ExprSingle::Path(ast::PathExpr {
        steps: vec![ast::StepExpr::AxisStep(ast::AxisStep {
            axis: ast::Axis::Self_,
            node_test: ast::NodeTest::KindTest(ast::KindTest::Any),
            predicates: vec![],
        })
        .with_empty_span()],
    })
    .with_empty_span();

    ast::StepExpr::PrimaryExpr(
        ast::PrimaryExpr::FunctionCall(ast::FunctionCall {
            name: ast::Name::new("root".to_string(), FN_NAMESPACE.to_string(), String::new())
                .with_empty_span(),
            arguments: vec![path_arg],
        })
        .with_empty_span(),
    )
    .with_span(span)
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ArgumentOrPlaceholder {
    Argument(ast::ExprSingleS),
    Placeholder,
}

// given a list of entries, each an argument or a placeholder, split this into
// a list of real arguments and a list of parameters to construct for the new
// function without the placeholders. If this list of parameters is empty, no
// wrapping placeholder function is constructed.
fn placeholder_arguments(
    aps: &[ArgumentOrPlaceholder],
) -> (Vec<ast::ExprSingleS>, Vec<ast::Param>) {
    let mut placeholder_index = 0;
    let mut arguments = Vec::new();
    let mut params = Vec::new();
    for argument_or_placeholder in aps.iter() {
        match argument_or_placeholder {
            ArgumentOrPlaceholder::Argument(expr) => {
                arguments.push(expr.clone());
            }
            ArgumentOrPlaceholder::Placeholder => {
                // XXX what if someone uses this as a parameter name?
                let param_name = format!("placeholder{}", placeholder_index);
                placeholder_index += 1;
                let name = ast::Name::name(&param_name);
                let param = ast::Param {
                    name: name.clone(),
                    type_: None,
                };
                params.push(param);
                arguments.push(
                    ast::ExprSingle::Path(ast::PathExpr {
                        steps: vec![ast::StepExpr::PrimaryExpr(
                            ast::PrimaryExpr::VarRef(name).with_empty_span(),
                        )
                        .with_empty_span()],
                    })
                    .with_empty_span(),
                );
            }
        }
    }
    (arguments, params)
}

// construct an inline function that calls the underlying
// function with the reduced placeholdered params
fn placeholder_wrapper_function(
    step_expr: ast::StepExprS,
    params: Vec<ast::Param>,
    span: Span,
) -> ast::PrimaryExprS {
    let path_expr = ast::PathExpr {
        steps: vec![step_expr],
    };
    let expr_single = ast::ExprSingle::Path(path_expr).with_empty_span();
    let body = Some(ast::Expr(vec![expr_single])).with_empty_span();
    ast::PrimaryExpr::InlineFunction(ast::InlineFunction {
        params,
        return_type: None,
        body,
        wrapper: true,
    })
    .with_span(span)
}
