use blanket::blanket;

use super::ast_core as ast;

#[blanket(default = "visit")]
pub(crate) trait AstVisitor {
    fn visit_xpath(&mut self, xpath: &mut ast::XPath);
    fn visit_expr(&mut self, expr: &mut ast::ExprS);
    fn visit_expr_or_empty(&mut self, expr: &mut ast::ExprOrEmptyS);
    fn visit_expr_single(&mut self, expr: &mut ast::ExprSingleS);
    fn visit_path_expr(&mut self, expr: &mut ast::PathExpr);
    fn visit_apply_expr(&mut self, expr: &mut ast::ApplyExpr);
    fn visit_let_expr(&mut self, expr: &mut ast::LetExpr);
    fn visit_if_expr(&mut self, expr: &mut ast::IfExpr);
    fn visit_binary_expr(&mut self, expr: &mut ast::BinaryExpr);
    fn visit_for_expr(&mut self, expr: &mut ast::ForExpr);
    fn visit_quantified_expr(&mut self, expr: &mut ast::QuantifiedExpr);
    fn visit_step_expr(&mut self, expr: &mut ast::StepExprS);
    fn visit_apply_operator(&mut self, expr: &mut ast::ApplyOperator);
    fn visit_var_ref(&mut self, expr: &mut ast::Name);
    fn visit_var_binding(&mut self, ast: &mut ast::Name);
    fn visit_function_name(&mut self, ast: &mut ast::Name);
    fn visit_binary_operator(&mut self, expr: &mut ast::BinaryOperator);
    fn visit_quantifier(&mut self, expr: &mut ast::Quantifier);
    fn visit_primary_expr(&mut self, expr: &mut ast::PrimaryExprS);
    fn visit_postfix(&mut self, expr: &mut ast::Postfix);
    fn visit_axis_step(&mut self, expr: &mut ast::AxisStep);
    fn visit_literal(&mut self, expr: &mut ast::Literal);
    fn visit_context_item(&mut self);
    fn visit_function_call(&mut self, expr: &mut ast::FunctionCall);
    fn visit_named_function_ref(&mut self, expr: &mut ast::NamedFunctionRef);
    fn visit_inline_function(&mut self, expr: &mut ast::InlineFunction);
    fn visit_map_constructor(&mut self, expr: &mut ast::MapConstructor);
    fn visit_map_constructor_entry(&mut self, expr: &mut ast::MapConstructorEntry);
    fn visit_array_constructor(&mut self, expr: &mut ast::ArrayConstructor);
    fn visit_unary_lookup(&mut self, expr: &mut ast::KeySpecifier);
    fn visit_param(&mut self, expr: &mut ast::Param);
    fn visit_sequence_type(&mut self, expr: &mut Option<ast::SequenceType>);
    fn visit_simple_map(&mut self, expr: &mut [ast::PathExpr]);
    fn visit_predicate(&mut self, expr: &mut ast::ExprS);
    fn visit_argument_list(&mut self, expr: &mut [ast::ExprSingleS]);
    fn visit_key_specifier(&mut self, expr: &mut ast::KeySpecifier);
    fn visit_axis(&mut self, expr: &mut ast::Axis);
    fn visit_node_test(&mut self, expr: &mut ast::NodeTest);
    fn visit_name_test(&mut self, expr: &mut ast::NameTest);
    fn visit_kind_test(&mut self, expr: &mut ast::KindTest);
}

pub(crate) mod visit {
    use super::AstVisitor;
    use crate::ast;

    pub(crate) fn visit_xpath<V: AstVisitor + ?Sized>(v: &mut V, xpath: &mut ast::XPath) {
        v.visit_expr(&mut xpath.0)
    }

    pub(crate) fn visit_expr<V: AstVisitor + ?Sized>(v: &mut V, expr: &mut ast::ExprS) {
        for expr_single in expr.value.0.iter_mut() {
            v.visit_expr_single(expr_single);
        }
    }

    pub(crate) fn visit_expr_or_empty<V: AstVisitor + ?Sized>(
        v: &mut V,
        expr: &mut ast::ExprOrEmptyS,
    ) {
        if let Some(expr) = &mut expr.value {
            for expr_single in expr.0.iter_mut() {
                v.visit_expr_single(expr_single);
            }
        }
    }

    pub(crate) fn visit_expr_single<V: AstVisitor + ?Sized>(
        v: &mut V,
        expr: &mut ast::ExprSingleS,
    ) {
        match &mut expr.value {
            ast::ExprSingle::Path(path_expr) => v.visit_path_expr(path_expr),
            ast::ExprSingle::Apply(apply_expr) => v.visit_apply_expr(apply_expr),
            ast::ExprSingle::Let(let_expr) => v.visit_let_expr(let_expr),
            ast::ExprSingle::If(if_expr) => v.visit_if_expr(if_expr),
            ast::ExprSingle::Binary(binary_expr) => v.visit_binary_expr(binary_expr),
            ast::ExprSingle::For(for_expr) => v.visit_for_expr(for_expr),
            ast::ExprSingle::Quantified(quantified_expr) => {
                v.visit_quantified_expr(quantified_expr)
            }
        }
    }

    pub(crate) fn visit_path_expr<V: AstVisitor + ?Sized>(v: &mut V, expr: &mut ast::PathExpr) {
        for step in expr.steps.iter_mut() {
            v.visit_step_expr(step);
        }
    }

    pub(crate) fn visit_apply_expr<V: AstVisitor + ?Sized>(v: &mut V, expr: &mut ast::ApplyExpr) {
        v.visit_path_expr(&mut expr.path_expr);
        v.visit_apply_operator(&mut expr.operator);
    }

    pub(crate) fn visit_apply_operator<V: AstVisitor + ?Sized>(
        v: &mut V,
        expr: &mut ast::ApplyOperator,
    ) {
        match expr {
            ast::ApplyOperator::SimpleMap(exprs) => {
                v.visit_simple_map(exprs);
            }
            ast::ApplyOperator::Unary(_unary_operators) => {}
            ast::ApplyOperator::Cast(_single_type) => {}
            ast::ApplyOperator::Castable(_single_type) => {}
            ast::ApplyOperator::Treat(_single_type) => {}
            ast::ApplyOperator::InstanceOf(_sequence_type) => {}
        }
    }
    pub(crate) fn visit_simple_map<V: AstVisitor + ?Sized>(v: &mut V, expr: &mut [ast::PathExpr]) {
        for expr in expr.iter_mut() {
            v.visit_path_expr(expr);
        }
    }

    pub(crate) fn visit_let_expr<V: AstVisitor + ?Sized>(v: &mut V, expr: &mut ast::LetExpr) {
        v.visit_var_binding(&mut expr.var_name.value);
        v.visit_expr_single(&mut expr.var_expr);
        v.visit_expr_single(&mut expr.return_expr);
    }

    pub(crate) fn visit_if_expr<V: AstVisitor + ?Sized>(v: &mut V, expr: &mut ast::IfExpr) {
        v.visit_expr(&mut expr.condition);
        v.visit_expr_single(&mut expr.then);
        v.visit_expr_single(&mut expr.else_);
    }

    pub(crate) fn visit_binary_expr<V: AstVisitor + ?Sized>(v: &mut V, expr: &mut ast::BinaryExpr) {
        v.visit_binary_operator(&mut expr.operator);
        v.visit_path_expr(&mut expr.left);
        v.visit_path_expr(&mut expr.right);
    }

    pub(crate) fn visit_binary_operator<V: AstVisitor + ?Sized>(
        _v: &mut V,
        _expr: &mut ast::BinaryOperator,
    ) {
        // intentionally left blank
    }

    pub(crate) fn visit_for_expr<V: AstVisitor + ?Sized>(v: &mut V, expr: &mut ast::ForExpr) {
        v.visit_var_binding(&mut expr.var_name.value);
        v.visit_expr_single(&mut expr.var_expr);
        v.visit_expr_single(&mut expr.return_expr);
    }

    pub(crate) fn visit_quantified_expr<V: AstVisitor + ?Sized>(
        v: &mut V,
        expr: &mut ast::QuantifiedExpr,
    ) {
        v.visit_quantifier(&mut expr.quantifier);
        v.visit_var_binding(&mut expr.var_name.value);
        v.visit_expr_single(&mut expr.var_expr);
        v.visit_expr_single(&mut expr.satisfies_expr);
    }

    pub(crate) fn visit_quantifier<V: AstVisitor + ?Sized>(
        _v: &mut V,
        _expr: &mut ast::Quantifier,
    ) {
        // intentionally left blank
    }

    pub(crate) fn visit_step_expr<V: AstVisitor + ?Sized>(v: &mut V, expr: &mut ast::StepExprS) {
        match &mut expr.value {
            ast::StepExpr::PrimaryExpr(primary_expr) => v.visit_primary_expr(primary_expr),
            ast::StepExpr::PostfixExpr { primary, postfixes } => {
                v.visit_primary_expr(primary);
                for postfix in postfixes {
                    v.visit_postfix(postfix);
                }
            }
            ast::StepExpr::AxisStep(axis_step) => v.visit_axis_step(axis_step),
        }
    }

    pub(crate) fn visit_postfix<V: AstVisitor + ?Sized>(v: &mut V, expr: &mut ast::Postfix) {
        match expr {
            ast::Postfix::Predicate(predicate) => {
                v.visit_predicate(predicate);
            }
            ast::Postfix::ArgumentList(argument_list) => {
                v.visit_argument_list(argument_list);
            }
            ast::Postfix::Lookup(key_specifier) => {
                v.visit_key_specifier(key_specifier);
            }
        }
    }

    pub(crate) fn visit_predicate<V: AstVisitor + ?Sized>(v: &mut V, expr: &mut ast::ExprS) {
        v.visit_expr(expr);
    }

    pub(crate) fn visit_argument_list<V: AstVisitor + ?Sized>(
        v: &mut V,
        expr: &mut [ast::ExprSingleS],
    ) {
        for argument in expr.iter_mut() {
            v.visit_expr_single(argument);
        }
    }

    pub(crate) fn visit_key_specifier<V: AstVisitor + ?Sized>(
        v: &mut V,
        key_specifier: &mut ast::KeySpecifier,
    ) {
        match key_specifier {
            ast::KeySpecifier::Expr(expr) => v.visit_expr_or_empty(expr),
            ast::KeySpecifier::NcName(_) => {}
            ast::KeySpecifier::Integer(_) => {}
            ast::KeySpecifier::Star => {}
        }
    }

    pub(crate) fn visit_axis_step<V: AstVisitor + ?Sized>(v: &mut V, expr: &mut ast::AxisStep) {
        v.visit_axis(&mut expr.axis);
        v.visit_node_test(&mut expr.node_test);
        for predicate in expr.predicates.iter_mut() {
            v.visit_expr(predicate);
        }
    }

    pub(crate) fn visit_axis<V: AstVisitor + ?Sized>(_v: &mut V, _expr: &mut ast::Axis) {
        // intentionally left blank
    }

    pub(crate) fn visit_node_test<V: AstVisitor + ?Sized>(v: &mut V, expr: &mut ast::NodeTest) {
        match expr {
            ast::NodeTest::NameTest(name_test) => {
                v.visit_name_test(name_test);
            }
            ast::NodeTest::KindTest(kind_test) => {
                v.visit_kind_test(kind_test);
            }
        }
    }

    pub(crate) fn visit_name_test<V: AstVisitor + ?Sized>(_v: &mut V, _expr: &mut ast::NameTest) {
        // intentionally left blank
    }

    pub(crate) fn visit_kind_test<V: AstVisitor + ?Sized>(_v: &mut V, _expr: &mut ast::KindTest) {
        // intentionally left blank
        // TODO: may need sub visitors
    }

    pub(crate) fn visit_primary_expr<V: AstVisitor + ?Sized>(
        v: &mut V,
        expr: &mut ast::PrimaryExprS,
    ) {
        match &mut expr.value {
            ast::PrimaryExpr::Literal(literal) => {
                v.visit_literal(literal);
            }
            ast::PrimaryExpr::VarRef(var_ref) => {
                v.visit_var_ref(var_ref);
            }
            ast::PrimaryExpr::Expr(expr) => {
                v.visit_expr_or_empty(expr);
            }
            ast::PrimaryExpr::ContextItem => {
                v.visit_context_item();
            }
            ast::PrimaryExpr::FunctionCall(function_call) => {
                v.visit_function_call(function_call);
            }
            ast::PrimaryExpr::NamedFunctionRef(named_function_ref) => {
                v.visit_named_function_ref(named_function_ref);
            }
            ast::PrimaryExpr::InlineFunction(inline_function) => {
                v.visit_inline_function(inline_function);
            }
            ast::PrimaryExpr::MapConstructor(map_constructor) => {
                v.visit_map_constructor(map_constructor);
            }
            ast::PrimaryExpr::ArrayConstructor(array_constructor) => {
                v.visit_array_constructor(array_constructor);
            }
            ast::PrimaryExpr::UnaryLookup(unary_lookup) => {
                v.visit_unary_lookup(unary_lookup);
            }
        }
    }

    pub(crate) fn visit_literal<V: AstVisitor + ?Sized>(_v: &mut V, _literal: &mut ast::Literal) {
        // intentionally left blank
    }

    pub(crate) fn visit_context_item<V: AstVisitor + ?Sized>(_v: &mut V) {
        // intentionally left blank
    }

    pub(crate) fn visit_var_ref<V: AstVisitor + ?Sized>(_v: &mut V, _var_ref: &mut ast::Name) {
        // intentionally left blank
    }

    pub(crate) fn visit_var_binding<V: AstVisitor + ?Sized>(_v: &mut V, _var_name: &mut ast::Name) {
        // intentionally left blank
    }

    pub(crate) fn visit_function_name<V: AstVisitor + ?Sized>(
        _v: &mut V,
        _function_name: &mut ast::Name,
    ) {
        // intentionally left blank
    }

    pub(crate) fn visit_function_call<V: AstVisitor + ?Sized>(
        v: &mut V,
        function_call: &mut ast::FunctionCall,
    ) {
        v.visit_function_name(&mut function_call.name.value);
        for arg in function_call.arguments.iter_mut() {
            v.visit_expr_single(arg);
        }
    }

    pub(crate) fn visit_named_function_ref<V: AstVisitor + ?Sized>(
        v: &mut V,
        named_function_ref: &mut ast::NamedFunctionRef,
    ) {
        v.visit_function_name(&mut named_function_ref.name.value);
    }

    pub(crate) fn visit_inline_function<V: AstVisitor + ?Sized>(
        v: &mut V,
        inline_function: &mut ast::InlineFunction,
    ) {
        for param in inline_function.params.iter_mut() {
            v.visit_param(param);
        }
        v.visit_sequence_type(&mut inline_function.return_type);
        v.visit_expr_or_empty(&mut inline_function.body);
    }

    pub(crate) fn visit_param<V: AstVisitor + ?Sized>(v: &mut V, param: &mut ast::Param) {
        v.visit_var_binding(&mut param.name);
        v.visit_sequence_type(&mut param.type_);
    }

    pub(crate) fn visit_map_constructor<V: AstVisitor + ?Sized>(
        v: &mut V,
        map_constructor: &mut ast::MapConstructor,
    ) {
        for entry in map_constructor.entries.iter_mut() {
            v.visit_map_constructor_entry(entry);
        }
    }

    pub(crate) fn visit_map_constructor_entry<V: AstVisitor + ?Sized>(
        v: &mut V,
        map_constructor_entry: &mut ast::MapConstructorEntry,
    ) {
        v.visit_expr_single(&mut map_constructor_entry.key);
        v.visit_expr_single(&mut map_constructor_entry.value);
    }

    pub(crate) fn visit_array_constructor<V: AstVisitor + ?Sized>(
        v: &mut V,
        array_constructor: &mut ast::ArrayConstructor,
    ) {
        match array_constructor {
            ast::ArrayConstructor::Square(expr) => v.visit_expr(expr),
            ast::ArrayConstructor::Curly(expr_or_empty) => v.visit_expr_or_empty(expr_or_empty),
        }
    }

    pub(crate) fn visit_unary_lookup<V: AstVisitor + ?Sized>(
        v: &mut V,
        key_specifier: &mut ast::KeySpecifier,
    ) {
        v.visit_key_specifier(key_specifier)
    }

    pub(crate) fn visit_sequence_type<V: AstVisitor + ?Sized>(
        _v: &mut V,
        _sequence_type: &mut Option<ast::SequenceType>,
    ) {
        // TODO
    }
}
