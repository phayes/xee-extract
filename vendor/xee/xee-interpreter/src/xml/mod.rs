/// XML integration.
mod document_order;
mod base;
mod document;
mod kind_test;
mod step;

pub(crate) use document_order::DocumentOrderAccess;
pub(crate) use base::BaseUriResolver;
pub use document::{Document, DocumentHandle, Documents, DocumentsError};
pub(crate) use kind_test::kind_test;
pub(crate) use step::resolve_step;
pub use step::Step;
