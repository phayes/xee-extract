pub mod ast;
mod context;
mod error;
mod operator;
mod parser;
pub mod pattern;
mod pattern_transform;
pub mod span;

pub use xee_name::{Name, NamespaceLookup, Namespaces, VariableNames, FN_NAMESPACE, XS_NAMESPACE};

pub use context::XPathParserContext;
pub use error::ParserError;
pub use parser::{parse_item_type, parse_kind_test, parse_name, parse_sequence_type};
pub use pattern::Pattern;
pub use span::WithSpan;
