use std::path::PathBuf;

use crate::metadata::Metadata;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Resource {
    metadata: Metadata,
    file: Option<PathBuf>,
    uri: String,
    media_type: Option<String>,
    encoding: Option<String>,
}
