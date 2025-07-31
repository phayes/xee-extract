use xee_xpath_ast::ast;

/// A span in the source code.
///
/// Designates where in the source code a certain error occurred.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct SourceSpan(usize, usize);

impl SourceSpan {
    pub(crate) fn entire(src: &str) -> Self {
        Self(0, src.len())
    }

    pub(crate) fn empty() -> Self {
        Self(0, 0)
    }

    /// Get the range of the span.
    pub fn range(&self) -> std::ops::Range<usize> {
        self.0..self.1
    }
}

impl From<ast::Span> for SourceSpan {
    fn from(span: ast::Span) -> Self {
        Self(span.start, span.end)
    }
}

impl From<std::ops::Range<usize>> for SourceSpan {
    fn from(range: std::ops::Range<usize>) -> Self {
        Self(range.start, range.end)
    }
}
