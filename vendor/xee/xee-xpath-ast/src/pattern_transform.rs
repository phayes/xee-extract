use crate::pattern;

struct Transformer<S, T, E, F>
where
    F: FnMut(&S) -> Result<T, E>,
{
    _s: std::marker::PhantomData<S>,
    _t: std::marker::PhantomData<T>,
    _e: std::marker::PhantomData<E>,
    transform: F,
}

impl<S, T, E, F> Transformer<S, T, E, F>
where
    F: FnMut(&S) -> Result<T, E>,
{
    fn new(transform: F) -> Self {
        Self {
            _s: std::marker::PhantomData,
            _t: std::marker::PhantomData,
            _e: std::marker::PhantomData,
            transform,
        }
    }

    fn pattern(&mut self, pattern: &pattern::Pattern<S>) -> Result<pattern::Pattern<T>, E> {
        Ok(match pattern {
            pattern::Pattern::Predicate(predicate_pattern) => {
                pattern::Pattern::Predicate(self.predicate_pattern(predicate_pattern)?)
            }
            pattern::Pattern::Expr(expr_pattern) => {
                pattern::Pattern::Expr(self.expr_pattern(expr_pattern)?)
            }
        })
    }

    fn predicates(&mut self, predicates: &[S]) -> Result<Vec<T>, E> {
        predicates
            .iter()
            .map(|predicate| (self.transform)(predicate))
            .collect::<Result<Vec<_>, _>>()
    }

    fn predicate_pattern(
        &mut self,
        predicate_pattern: &pattern::PredicatePattern<S>,
    ) -> Result<pattern::PredicatePattern<T>, E> {
        Ok(pattern::PredicatePattern {
            predicates: self.predicates(&predicate_pattern.predicates)?,
        })
    }

    fn expr_pattern(
        &mut self,
        expr_pattern: &pattern::ExprPattern<S>,
    ) -> Result<pattern::ExprPattern<T>, E> {
        Ok(match expr_pattern {
            pattern::ExprPattern::Path(path_expr) => {
                pattern::ExprPattern::Path(self.path_expr(path_expr)?)
            }
            pattern::ExprPattern::BinaryExpr(binary_expr) => self.binary_expr(binary_expr)?,
        })
    }

    fn path_expr(&mut self, path_expr: &pattern::PathExpr<S>) -> Result<pattern::PathExpr<T>, E> {
        Ok(pattern::PathExpr {
            root: self.path_root(&path_expr.root)?,
            steps: path_expr
                .steps
                .iter()
                .map(|step| self.step_expr(step))
                .collect::<Result<Vec<_>, _>>()?,
        })
    }

    fn path_root(&mut self, path_root: &pattern::PathRoot<S>) -> Result<pattern::PathRoot<T>, E> {
        Ok(match path_root {
            pattern::PathRoot::Rooted { root, predicates } => pattern::PathRoot::Rooted {
                root: root.clone(),
                predicates: self.predicates(predicates)?,
            },
            pattern::PathRoot::AbsoluteSlash => pattern::PathRoot::AbsoluteSlash,
            pattern::PathRoot::AbsoluteDoubleSlash => pattern::PathRoot::AbsoluteDoubleSlash,
            pattern::PathRoot::Relative => pattern::PathRoot::Relative,
        })
    }

    fn step_expr(&mut self, step_expr: &pattern::StepExpr<S>) -> Result<pattern::StepExpr<T>, E> {
        Ok(match step_expr {
            pattern::StepExpr::PostfixExpr(postfix_expr) => {
                pattern::StepExpr::PostfixExpr(self.postfix_expr(postfix_expr)?)
            }
            pattern::StepExpr::AxisStep(axis_step) => {
                pattern::StepExpr::AxisStep(self.axis_step(axis_step)?)
            }
        })
    }

    fn postfix_expr(
        &mut self,
        postfix_expr: &pattern::PostfixExpr<S>,
    ) -> Result<pattern::PostfixExpr<T>, E> {
        Ok(pattern::PostfixExpr {
            expr: self.expr_pattern(&postfix_expr.expr)?,
            predicates: self.predicates(&postfix_expr.predicates)?,
        })
    }

    fn axis_step(&mut self, axis_step: &pattern::AxisStep<S>) -> Result<pattern::AxisStep<T>, E> {
        Ok(pattern::AxisStep {
            forward: axis_step.forward,
            node_test: axis_step.node_test.clone(),
            predicates: self.predicates(&axis_step.predicates)?,
        })
    }

    fn binary_expr(
        &mut self,
        binary_expr: &pattern::BinaryExpr<S>,
    ) -> Result<pattern::ExprPattern<T>, E> {
        Ok(pattern::ExprPattern::BinaryExpr(pattern::BinaryExpr {
            operator: binary_expr.operator,
            left: Box::new(self.expr_pattern(&binary_expr.left)?),
            right: Box::new(self.expr_pattern(&binary_expr.right)?),
        }))
    }
}

pub fn transform_pattern<S, T, E, F>(
    pattern: &pattern::Pattern<S>,
    transform: F,
) -> Result<pattern::Pattern<T>, E>
where
    F: FnMut(&S) -> Result<T, E>,
{
    Transformer::new(transform).pattern(pattern)
}
