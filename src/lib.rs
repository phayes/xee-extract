//! XPath-driven deserialization crate using Xee as the underlying engine.
//!
//! This crate provides a procedural macro `Extract` that allows you to
//! deserialize XML documents into Rust structs using XPath expressions.

use std::str::FromStr;
use xee_xpath::{Documents, Item};

// Re-export the macro
pub use xee_extract_macros::Extract;

// Error types module
pub mod error;

// Re-export error types
pub use error::{Error, ExtractError};

pub trait Extract: Sized {
    /// Extract from an XML node (for recursive extraction)
    fn extract_from_node(documents: &mut Documents, item: &Item, extract_id: Option<&str>) -> Result<Self, Error> {
        match item {
            Item::Node(_) => {
                Self::extract(documents, item, extract_id)
            }
            _ => {
                return Result::Err(Error::InvalidXPath(
                    "extract targets non-node items".to_string(),
                ));
            }
        }
    }

    /// Extract from a context item using XPath expressions relative to that item
    fn extract(documents: &mut Documents, context_item: &Item, extract_id: Option<&str>) -> Result<Self, Error>;
}

/// Trait for deserializing a type from an XPath item
pub trait ExtractValue: Sized {
    /// Deserialize a value from an XPath item
    fn extract_value(documents: &mut Documents, item: &Item) -> Result<Self, Error>;
}

/// Default ExtractValue impl that punts to FromStr
impl<T> ExtractValue for T
where
    T: FromStr,
    T::Err: std::fmt::Display,
{
    fn extract_value(documents: &mut Documents, item: &Item) -> Result<Self, Error> {
        let s = item.string_value(documents.xot())?;
        s.parse::<T>()
            .map_err(|e| Error::DeserializationError(e.to_string()))
    }
}





/// Extractor for XML documents using XPath expressions
pub struct Extractor {
    pub variables: std::collections::HashMap<String, String>,
    pub extract_name: Option<String>,
}

impl Default for Extractor {
    fn default() -> Self {
        Self {
            variables: std::collections::HashMap::new(),
            extract_name: None,
        }
    }
}

impl Extractor {
    /// Create a new extractor
    pub fn new() -> Self {
        Self::default()
    }

    pub fn named(name: &str) -> Self {
        Self {
            variables: std::collections::HashMap::new(),
            extract_name: Some(name.to_string()),
        }
    }

    /// Add a variable to the extractor
    pub fn with_variable(mut self, name: &str, value: &str) -> Self {
        self.variables.insert(name.to_string(), value.to_string());
        self
    }

    /// Extract a single struct from an XML document
    pub fn extract_one<T>(&self, xml: &str) -> Result<T, ExtractError>
    where
        T: Extract,
    {
        let mut documents = xee_xpath::Documents::new();
        let doc = documents.add_string_without_uri(xml).map_err(|e| ExtractError::new(Error::DocumentsError(e), &xml))?;

        use xee_xpath::Itemable;
        let item = doc.to_item(&mut documents).map_err(|e| ExtractError::new(Error::SpannedError(e), &xml))?;

        // Use the trait's deserialize method
        let res = T::extract(&mut documents, &item, self.extract_name.as_deref());

        match res {
            Ok(value) => Ok(value),
            Err(error) => Err(ExtractError::new(error, &xml)),
        }
    }
}
