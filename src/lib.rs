//! XPath-driven deserialization crate using Xee as the underlying engine.
//!
//! This crate provides a procedural macro `Extract` that allows you to
//! deserialize XML documents into Rust structs using XPath expressions.

use std::str::FromStr;
use xee_xpath::{Documents, Item};

// Re-export the macro
pub use xee_extract_macros::Extract;

pub trait Extract: Sized {
    ///
    /// Extract from an XML node (for recursive extraction)
    fn extract_from_node(documents: &mut Documents, item: &Item) -> Result<Self, Error> {
        // For Node items, use the more efficient context-based extraction
        match item {
            Item::Node(_) => {
                // Use the new context-based extraction method
                Self::extract(documents, item)
            }
            _ => {
                return Result::Err(Error::InvalidXPath(
                    "extract targets non-node items".to_string(),
                ));
            }
        }
    }

    /// Extract from a context item using XPath expressions relative to that item
    /// This is more efficient than extract_from_node as it doesn't require
    /// serialization to XML string and re-parsing
    fn extract(documents: &mut Documents, context_item: &Item) -> Result<Self, Error> {
        // For now, fall back to the existing implementation
        // This will be overridden by the macro to use context-based extraction
        Self::extract_from_node(documents, context_item)
    }
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

/// A user-friendly error type that provides pretty error messages with XML context
#[derive(Debug)]
pub struct ExtractorError {
    /// The underlying core error
    pub error: Error,
    /// The original XML document
    pub xml: String,
    /// Optional span information for highlighting the error location
    pub span: Option<xee_interpreter::span::SourceSpan>,
    /// Additional context about where the error occurred
    pub context: Option<String>,
}

impl ExtractorError {
    /// Create a new ExtractorError from a core Error and XML document
    pub fn new(error: Error, xml: String) -> Self {
        Self {
            error,
            xml,
            span: None,
            context: None,
        }
    }

    /// Create a new ExtractorError with span information
    pub fn with_span(mut self, span: xee_interpreter::span::SourceSpan) -> Self {
        self.span = Some(span);
        self
    }

    /// Create a new ExtractorError with additional context
    pub fn with_context(mut self, context: String) -> Self {
        self.context = Some(context);
        self
    }

    /// Extract the XML snippet around the error location if span is available
    fn extract_xml_context(&self) -> Option<String> {
        let span = self.span?;
        let range = span.range();
        
        // Find the start and end of the XML element containing the error
        let xml_bytes = self.xml.as_bytes();
        let start = range.start.saturating_sub(200); // Look back 200 bytes
        let end = (range.end + 200).min(xml_bytes.len()); // Look forward 200 bytes
        
        // Find the nearest complete XML element boundaries
        let mut actual_start = start;
        let mut actual_end = end;
        
        // Look for the start of an XML element before the error
        for i in (start..range.start).rev() {
            if i < xml_bytes.len() && xml_bytes[i] == b'<' {
                actual_start = i;
                break;
            }
        }
        
        // Look for the end of an XML element after the error
        for i in range.end..end {
            if i < xml_bytes.len() && xml_bytes[i] == b'>' {
                actual_end = i + 1;
                break;
            }
        }
        
        // Extract the XML snippet
        if actual_start < actual_end && actual_end <= xml_bytes.len() {
            let snippet = &xml_bytes[actual_start..actual_end];
            let snippet_str = String::from_utf8_lossy(snippet).to_string();
            
            // If the snippet is too short, try to get more context
            if snippet_str.len() < 50 {
                let expanded_start = actual_start.saturating_sub(100);
                let expanded_end = (actual_end + 100).min(xml_bytes.len());
                if expanded_start < expanded_end && expanded_end <= xml_bytes.len() {
                    let expanded_snippet = &xml_bytes[expanded_start..expanded_end];
                    return String::from_utf8_lossy(expanded_snippet).to_string().into();
                }
            }
            
            snippet_str.into()
        } else {
            None
        }
    }

    /// Generate a pretty error message with XML context
    pub fn pretty_message(&self) -> String {
        let mut message = String::new();
        
        // Add the main error message
        match &self.error {
            Error::InvalidXPath(msg) => {
                message.push_str(&format!("Invalid XPath expression: {}\n", msg));
            }
            Error::DeserializationError(msg) => {
                message.push_str(&format!("Deserialization error: {}\n", msg));
            }
            Error::SpannedError(spanned_err) => {
                message.push_str(&format!("XPath error: {}\n", spanned_err.error.message()));
                let note = spanned_err.error.note();
                if !note.is_empty() {
                    message.push_str(&format!("Note: {}\n", note));
                }
            }
            Error::XeeInterpreterError(err) => {
                message.push_str(&format!("XPath error: {}\n", err.message()));
                let note = err.note();
                if !note.is_empty() {
                    message.push_str(&format!("Note: {}\n", note));
                }
            }
            Error::DocumentsError(err) => {
                message.push_str(&format!("XML document error: {}\n", err));
            }
        }
        
        // Add context if available
        if let Some(context) = &self.context {
            message.push_str(&format!("Context: {}\n", context));
        }
        
        // Add XML context if span is available
        if let Some(xml_snippet) = self.extract_xml_context() {
            message.push_str("\nRelevant XML context:\n");
            message.push_str("```xml\n");
            message.push_str(&xml_snippet);
            message.push_str("\n```\n");
            
            // Add pointer to the error location if we can determine it
            if let Some(span) = self.span {
                let range = span.range();
                let snippet_start = self.xml.as_bytes()[..range.start]
                    .iter()
                    .filter(|&&b| b == b'\n')
                    .count();
                let error_line = snippet_start + 1;
                message.push_str(&format!("Error occurred around line {}\n", error_line));
            }
        } else {
            // If we don't have span info, show the first part of the XML for context
            let xml_preview = if self.xml.len() > 200 {
                format!("{}...", &self.xml[..200])
            } else {
                self.xml.clone()
            };
            
            if !xml_preview.trim().is_empty() {
                message.push_str("\nXML document:\n");
                message.push_str("```xml\n");
                message.push_str(&xml_preview);
                message.push_str("\n```\n");
            }
        }
        
        message
    }
}

impl std::error::Error for ExtractorError {}

impl std::fmt::Display for ExtractorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.pretty_message())
    }
}

/// Extractor for XML documents using XPath expressions
pub struct Extractor {
    pub variables: std::collections::HashMap<String, String>,
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
        T: Extract,
    {
        let mut documents = xee_xpath::Documents::new();
        let doc = documents.add_string_without_uri(xml)?;

        use xee_xpath::Itemable;
        let item = doc.to_item(&mut documents)?;

        // Use the trait's deserialize method
        T::extract(&mut documents, &item)
    }

    /// Extract a single struct from an XML document with pretty error messages
    pub fn extract_one_pretty<T>(&self, xml: &str) -> Result<T, ExtractorError>
    where
        T: Extract,
    {
        self.extract_one::<T>(xml).map_err(|error| {
            let mut extractor_error = ExtractorError::new(error, xml.to_string());
            
            // If the error has span information, include it
            match &extractor_error.error {
                Error::SpannedError(spanned_err) => {
                    if let Some(span) = spanned_err.span {
                        extractor_error = extractor_error.with_span(span);
                    }
                }
                Error::XeeInterpreterError(_) => {
                    // For interpreter errors, we might not have span info
                    // but we can still provide context
                }
                Error::DocumentsError(_) => {
                    // For document errors, we might not have span info
                    // but we can still provide context
                }
                _ => {}
            }
            
            extractor_error
        })
    }
}

impl Default for Extractor {
    fn default() -> Self {
        Self::new()
    }
}
