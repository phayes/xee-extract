//! XPath-driven deserialization crate using Xee as the underlying engine.
//!
//! This crate provides a procedural macro `XeeExtract` that allows you to
//! deserialize XML documents into Rust structs using XPath expressions.

use std::str::FromStr;
use xee_xpath::{Documents, Item};
use std::any::Any;

// Re-export the macro
pub use xee_extract_macros::XeeExtract;

pub trait XeeExtract: Sized {
    fn extract(xml: &str) -> Result<Self, Error>;
    
    /// Extract from an XML node (for recursive extraction)
    fn extract_from_node(documents: &mut Documents, item: &Item) -> Result<Self, Error> {
        // For Node items, use the more efficient context-based extraction
        match item {
            Item::Node(_) => {
                // Use the new context-based extraction method
                Self::extract_from_context(documents, item)
            }
            _ => {
                // For non-Node items, fall back to string conversion
                let xml_str = item.string_value(documents.xot())?;
                Self::extract(&xml_str)
            }
        }
    }

    /// Extract from a context item using XPath expressions relative to that item
    /// This is more efficient than extract_from_node as it doesn't require
    /// serialization to XML string and re-parsing
    fn extract_from_context(documents: &mut Documents, context_item: &Item) -> Result<Self, Error> {
        // For now, fall back to the existing implementation
        // This will be overridden by the macro to use context-based extraction
        Self::extract_from_node(documents, context_item)
    }
}

pub trait XeeExtractMarker: Any {
    fn as_any(&self) -> &dyn Any;
}

/// Helper function to check if a value is of a specific type
pub fn is_xee_extract<T: XeeExtractMarker + 'static>(x: &dyn XeeExtractMarker) -> bool {
    x.as_any().is::<T>()
}

/// Helper function to downcast a XeeExtractMarker to a specific type
pub fn downcast_xee_extract<T: XeeExtractMarker + 'static>(x: &dyn XeeExtractMarker) -> Option<&T> {
    x.as_any().downcast_ref::<T>()
}

/// Helper function to downcast a boxed XeeExtractMarker to a specific type
pub fn downcast_boxed_xee_extract<T: XeeExtractMarker + 'static>(x: Box<dyn XeeExtractMarker>) -> Result<Box<T>, Box<dyn XeeExtractMarker>> {
    if x.as_any().is::<T>() {
        // SAFETY: We just checked that x is of type T
        let raw = Box::into_raw(x);
        Ok(unsafe { Box::from_raw(raw as *mut T) })
    } else {
        Err(x)
    }
}

/// Trait for deserializing a type from an XPath item
pub trait XeeExtractDeserialize: Sized {
    /// Deserialize a value from an XPath item
    fn deserialize(
        documents: &mut Documents,
        item: &Item,
    ) -> Result<Self, Error>;

    fn as_xee_extract(&self) -> Option<&dyn XeeExtractMarker> {
        None
    }
}

/// Default XeeExtractDeserialize impl that punts to FromStr
impl<T> XeeExtractDeserialize for T
where
    T: FromStr,
    T::Err: std::fmt::Display,
{
    fn deserialize(
        documents: &mut Documents,
        item: &Item,
    ) -> Result<Self, Error> {
        let s = item.string_value(documents.xot())?;
        s.parse::<T>()
            .map_err(|e| Error::DeserializationError(e.to_string()))
    }
}

/// Extractor for XML documents using XPath expressions
pub struct Extractor {
    pub variables: std::collections::HashMap<String, String>,
}

#[derive(Debug)]
pub enum Error {
    InvalidXPath(String),
    DeserializationError(String),
    SpannedError(xee_interpreter::error::SpannedError),
    XeeInterpreterError(xee_interpreter::error::Error),
    DocumentsError(xee_xpath::error::DocumentsError),
}

impl From<xee_interpreter::error::SpannedError> for Error {
    fn from(err: xee_interpreter::error::SpannedError) -> Self {
        Error::SpannedError(err)
    }
}

impl From<xee_interpreter::error::Error> for Error {
    fn from(err: xee_interpreter::error::Error) -> Self {
        Error::XeeInterpreterError(err)
    }
}

impl From<xee_xpath::error::DocumentsError> for Error {
    fn from(err: xee_xpath::error::DocumentsError) -> Self {
        Error::DocumentsError(err)
    }
}

// Add conversion for xot serialize errors
impl From<xot::Error> for Error {
    fn from(_err: xot::Error) -> Self {
        Error::XeeInterpreterError(xee_interpreter::error::Error::FODC0002)
    }
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InvalidXPath(msg) => write!(f, "Invalid XPath: {}", msg),
            Error::DeserializationError(msg) => write!(f, "Deserialization error: {}", msg),
            Error::SpannedError(err) => write!(f, "Spanned error: {}", err),
            Error::XeeInterpreterError(err) => write!(f, "Xee interpreter error: {}", err),
            Error::DocumentsError(err) => write!(f, "Documents error: {}", err),
        }
    }
}

impl Extractor {
    /// Create a new extractor
    pub fn new() -> Self {
        Self {
            variables: std::collections::HashMap::new(),
        }
    }

    /// Add a variable to the extractor
    pub fn with_variable(mut self, name: &str, value: &str) -> Self {
        self.variables.insert(name.to_string(), value.to_string());
        self
    }

    /// Extract a single struct from an XML document
    pub fn extract_one<T>(&self, xml: &str) -> Result<T, Error>
    where
        T: XeeExtract,
    {
        // Use the trait's deserialize method
        T::extract(xml)
    }
}

impl Default for Extractor {
    fn default() -> Self {
        Self::new()
    }
}

