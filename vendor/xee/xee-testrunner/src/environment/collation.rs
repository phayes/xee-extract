#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Collation {
    pub(crate) uri: Option<String>,
    pub(crate) default: bool,
}
