/// String support for XPath. XPath allows strings to be compared
/// using collations.
mod collation;

pub use collation::Collation;
pub(crate) use collation::Collations;
