//! Context in order to construct and execute XPath queries.
//!
//! In order to construct an XPath query, you need a [`StaticContext`], which
//! has enough information to compile it.
//!
//! The higher level APIs in this crate create these contexts for you, but if
//! you need custom namespaces or variables in your XPath expressions,
//! you need to build the context.
//!
//! In order to execute an XPath query, you need to create a
//! [`DynamicContext`], which has enough information to execute it.
//!
//! Builder APIs are provided which you need to use in order to create
//! [`StaticContext`] and [`DynamicContext`].

pub use xee_interpreter::context::{
    DynamicContext, DynamicContextBuilder, StaticContext, StaticContextBuilder, Variables,
};
pub use xee_interpreter::string::Collation;
