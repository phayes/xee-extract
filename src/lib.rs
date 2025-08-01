//! XPath-driven deserialization crate using Xee as the underlying engine.
//!
//! This crate provides a procedural macro `XeeExtract` that allows you to
//! deserialize XML documents into Rust structs using XPath expressions.

use std::str::FromStr;
use xee_xpath::{Documents, Item};


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

/// Trait for deserializing a type from an XPath item
pub trait XeeExtractDeserialize: Sized {
    /// Deserialize a value from an XPath item
    fn deserialize(
        documents: &mut Documents,
        item: &Item,
    ) -> Result<Self, Error>;
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

// Re-export the macro
pub use xee_extract_macros::XeeExtract;

#[cfg(test)]
mod tests {
    use super::*;
    use crate as xee_extract;

    #[derive(XeeExtract, Debug, PartialEq)]
    struct SimpleStruct {
        #[xpath("//id/text()")]
        id: String,

        #[xpath("//title/text()")]
        title: String,

        #[xpath("//category/@term")]
        category: Option<String>,
    }

    #[derive(XeeExtract, Debug, PartialEq)]
    struct ComplexStruct {
        #[xpath("//id/text()")]
        id: String,

        #[xpath("//title/text()")]
        title: String,

        #[xpath("//subtitle/text()")]
        subtitle: Option<String>,

        #[xpath("//category/@term")]
        category: Option<String>,

        #[xpath("//tags/tag/text()")]
        tags: Vec<String>,
    }

    #[derive(XeeExtract, Debug, PartialEq)]
    struct NestedStruct {
        #[xpath("//id/text()")]
        id: String,

        #[xpath("//author/name/text()")]
        author_name: String,

        #[xpath("//author/email/text()")]
        author_email: Option<String>,
    }

    #[derive(XeeExtract, Debug, PartialEq)]
    struct NamespaceStruct {
        #[xpath("//atom:id/text()")]
        id: String,

        #[xpath("//atom:title/text()")]
        title: String,

        #[xpath("//atom:category/@term")]
        category: Option<String>,
    }

    #[test]
    fn test_simple_extraction() {
        let xml = r#"
            <entry>
                <id>123</id>
                <title>Sample Title</title>
                <category term="test"/>
            </entry>
        "#;
        
        let extractor = Extractor::new();
        let result: SimpleStruct = extractor.extract_one(xml).unwrap();
        
        assert_eq!(result.id, "123");
        assert_eq!(result.title, "Sample Title");
        assert_eq!(result.category, Some("test".to_string()));
    }

    #[test]
    fn test_extraction_with_missing_optional_field() {
        let xml = r#"
            <entry>
                <id>123</id>
                <title>Sample Title</title>
            </entry>
        "#;
        
        let extractor = Extractor::new();
        let result: SimpleStruct = extractor.extract_one(xml).unwrap();
        
        assert_eq!(result.id, "123");
        assert_eq!(result.title, "Sample Title");
        assert_eq!(result.category, None);
    }

    #[test]
    fn test_complex_extraction_with_vectors() {
        let xml = r#"
            <entry>
                <id>456</id>
                <title>Complex Title</title>
                <subtitle>Complex Subtitle</subtitle>
                <category term="complex"/>
                <tags>
                    <tag>rust</tag>
                    <tag>xpath</tag>
                    <tag>xml</tag>
                </tags>
            </entry>
        "#;
        
        let extractor = Extractor::new();
        let result: ComplexStruct = extractor.extract_one(xml).unwrap();
        
        assert_eq!(result.id, "456");
        assert_eq!(result.title, "Complex Title");
        assert_eq!(result.subtitle, Some("Complex Subtitle".to_string()));
        assert_eq!(result.category, Some("complex".to_string()));
        assert_eq!(result.tags, vec!["rust", "xpath", "xml"]);
    }

    #[test]
    fn test_nested_extraction() {
        let xml = r#"
            <entry>
                <id>789</id>
                <author>
                    <name>John Doe</name>
                    <email>john@example.com</email>
                </author>
            </entry>
        "#;
        
        let extractor = Extractor::new();
        let result: NestedStruct = extractor.extract_one(xml).unwrap();
        
        assert_eq!(result.id, "789");
        assert_eq!(result.author_name, "John Doe");
        assert_eq!(result.author_email, Some("john@example.com".to_string()));
    }

    #[test]
    fn test_nested_extraction_with_missing_optional() {
        let xml = r#"
            <entry>
                <id>789</id>
                <author>
                    <name>Jane Smith</name>
                </author>
            </entry>
        "#;
        
        let extractor = Extractor::new();
        let result: NestedStruct = extractor.extract_one(xml).unwrap();
        
        assert_eq!(result.id, "789");
        assert_eq!(result.author_name, "Jane Smith");
        assert_eq!(result.author_email, None);
    }

    #[test]
    fn test_extraction_with_variables() {
        let xml = r#"
            <entry>
                <id>urn:uuid:12345678-1234-1234-1234-123456789abc</id>
                <title>Sample Title</title>
                <category term="test"/>
            </entry>
        "#;
        
        let extractor = Extractor::new()
            .with_variable("env", "production")
            .with_variable("base_url", "https://api.example.com");
        
        // This test verifies that variables can be set (even if not used in this simple case)
        let result: SimpleStruct = extractor.extract_one(xml).unwrap();
        
        assert_eq!(result.id, "urn:uuid:12345678-1234-1234-1234-123456789abc");
        assert_eq!(result.title, "Sample Title");
        assert_eq!(result.category, Some("test".to_string()));
    }

    #[test]
    fn test_empty_vector_extraction() {
        let xml = r#"
            <entry>
                <id>123</id>
                <title>Title</title>
                <tags>
                </tags>
            </entry>
        "#;
        
        let extractor = Extractor::new();
        let result: ComplexStruct = extractor.extract_one(xml).unwrap();
        
        assert_eq!(result.id, "123");
        assert_eq!(result.title, "Title");
        assert_eq!(result.tags, Vec::<String>::new());
    }

    #[test]
    fn test_missing_required_field_error() {
        let xml = r#"
            <entry>
                <title>Sample Title</title>
                <category term="test"/>
            </entry>
        "#;
        
        let extractor = Extractor::new();
        let result: Result<SimpleStruct, Error> = extractor.extract_one(xml);
        
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_xml_error() {
        let xml = r#"
            <entry>
                <id>123</id>
                <title>Sample Title</title>
                <unclosed>
            </entry>
        "#;
        
        let extractor = Extractor::new();
        let result: Result<SimpleStruct, Error> = extractor.extract_one(xml);
        
        assert!(result.is_err());
    }

    #[test]
    fn test_extractor_with_multiple_variables() {
        let extractor = Extractor::new()
            .with_variable("var1", "value1")
            .with_variable("var2", "value2")
            .with_variable("var3", "value3");
        
        // Test that the extractor can be created with multiple variables
        assert_eq!(extractor.variables.len(), 3);
        assert_eq!(extractor.variables.get("var1"), Some(&"value1".to_string()));
        assert_eq!(extractor.variables.get("var2"), Some(&"value2".to_string()));
        assert_eq!(extractor.variables.get("var3"), Some(&"value3".to_string()));
    }
}
