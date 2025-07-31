#[cfg(test)]
mod tests {
    use insta::assert_debug_snapshot;

    use xee_interpreter::{context, error};
    use xee_ir::{ir, Variables};

    use crate::ast_ir::IrConverter;

    pub fn convert_ir(
        static_context: &context::StaticContext,
        xpath: &str,
    ) -> error::SpannedResult<ir::ExprS> {
        let ast = static_context.parse_xpath(xpath)?;
        let mut variables = Variables::new();
        let mut converter = IrConverter::new(&mut variables, static_context);
        converter.convert_xpath(&ast)
    }

    #[test]
    fn test_span_sequence_ir() {
        let expr = "(0, 1, 2)";
        //          012345678
        //          0       8
        //  So from 0, 9 is expected
        // lets examine the IR
        let static_context = context::StaticContext::default();
        assert_debug_snapshot!(convert_ir(&static_context, expr));
    }
}
