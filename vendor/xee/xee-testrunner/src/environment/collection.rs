use super::{resource::Resource, source::Source};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Collection {
    pub(crate) uri: String,
    pub(crate) queries: Vec<Query>,
    pub(crate) sources: Vec<Source>,
    // this doesn't appear to be in use by XPath or XSLT test suites
    pub(crate) resources: Vec<Resource>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Query {
    expression: String,
}
