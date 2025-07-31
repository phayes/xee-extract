use chumsky::span::SimpleSpan as SourceSpan;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Spanned<T> {
    pub value: T,
    pub span: SourceSpan,
}

pub trait WithSpan
where
    Self: Sized,
{
    fn with_span(self, span: SourceSpan) -> Spanned<Self> {
        Spanned { value: self, span }
    }
    fn with_empty_span(self) -> Spanned<Self> {
        self.with_span((0..0).into())
    }
}

// custom serializer that skips span, so we don't see it in the ron
// snapshot tests
#[cfg(feature = "serde")]
impl<T: serde::Serialize> serde::Serialize for Spanned<T> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.value.serialize(serializer)
    }
}

impl<T> Spanned<T> {
    pub fn new(value: T, span: SourceSpan) -> Self {
        Self { value, span }
    }

    pub fn map(self, f: impl FnOnce(T) -> T) -> Self {
        Self {
            value: f(self.value),
            span: self.span,
        }
    }
}

#[cfg(test)]
mod tests {
    // use insta::assert_debug_snapshot;

    // use crate::ast::parse_xpath;
    // use crate::namespaces::Namespaces;

    // #[test]
    // fn test_span_sequence_ast() {
    //     let expr = "(0, 1, 2)";
    //     //          012345678
    //     //          0       8
    //     //  So from 0, 9 is expected
    //     // let's examine the AST
    //     let namespaces = Namespaces::new(None, None);
    //     assert_debug_snapshot!(parse_xpath(expr, &namespaces, &[]));
    // }
}
