//! Error handling

pub use xee_interpreter::error::{
    Error as ErrorValue, Result as ValueResult, SpannedError as Error, SpannedResult as Result,
};
pub use xee_interpreter::span::SourceSpan;
pub use xee_interpreter::xml::DocumentsError;
