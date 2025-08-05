use xee_interpreter;
use xee_xpath;

// TODO: Top level error should be ExtractionError Struct, that has a lot of option fields that contains both field info and span info
// Then an internal error enum to hold other error types
// NoValueFoundError can then just become a variant of the internal error enum

/// Error type returned from `ExtractValue::extract_value` implementations.
#[derive(Debug)]
pub enum ErrorType {
    InvalidXPath(String),
    DeserializationError(String),
    UnknownExtractId(String),
    SpannedError(xee_interpreter::error::SpannedError),
    XeeInterpreterError(xee_interpreter::error::Error),
    DocumentsError(xee_xpath::error::DocumentsError),
}

impl From<xee_interpreter::error::SpannedError> for ErrorType {
    fn from(err: xee_interpreter::error::SpannedError) -> Self {
        ErrorType::SpannedError(err)
    }
}

impl From<xee_interpreter::error::Error> for ErrorType {
    fn from(err: xee_interpreter::error::Error) -> Self {
        ErrorType::XeeInterpreterError(err)
    }
}

impl From<xee_xpath::error::DocumentsError> for ErrorType {
    fn from(err: xee_xpath::error::DocumentsError) -> Self {
        ErrorType::DocumentsError(err)
    }
}

// Add conversion for xot serialize errors
impl From<xot::Error> for ErrorType {
    fn from(_err: xot::Error) -> Self {
        ErrorType::XeeInterpreterError(xee_interpreter::error::Error::FODC0002)
    }
}

impl std::error::Error for ErrorType {}

impl std::fmt::Display for ErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorType::InvalidXPath(msg) => write!(f, "Invalid XPath: {}", msg),
            ErrorType::DeserializationError(msg) => write!(f, "Deserialization error: {}", msg),
            ErrorType::UnknownExtractId(msg) => write!(f, "Unknown extract named: {}", msg),
            ErrorType::SpannedError(err) => write!(f, "Spanned error: {}", err),
            ErrorType::XeeInterpreterError(err) => write!(f, "Xee interpreter error: {}", err),
            ErrorType::DocumentsError(err) => write!(f, "Documents error: {}", err),
        }
    }
}

#[derive(Debug)]
pub enum Error {
    InvalidXPath(String),
    DeserializationError(String),
    UnknownExtractId(String),
    FieldExtract(FieldExtractionError),
    SpannedError(xee_interpreter::error::SpannedError),
    XeeInterpreterError(xee_interpreter::error::Error),
    DocumentsError(xee_xpath::error::DocumentsError),
}

impl From<ErrorType> for Error {
    fn from(err: ErrorType) -> Self {
        match err {
            ErrorType::InvalidXPath(msg) => Error::InvalidXPath(msg),
            ErrorType::DeserializationError(msg) => Error::DeserializationError(msg),
            ErrorType::UnknownExtractId(msg) => Error::UnknownExtractId(msg),
            ErrorType::SpannedError(e) => Error::SpannedError(e),
            ErrorType::XeeInterpreterError(e) => Error::XeeInterpreterError(e),
            ErrorType::DocumentsError(e) => Error::DocumentsError(e),
        }
    }
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
            Error::UnknownExtractId(msg) => write!(f, "Unknown extract named: {}", msg),
            Error::FieldExtract(err) => write!(f, "Field extraction error: {}", err),
            Error::SpannedError(err) => write!(f, "Spanned error: {}", err),
            Error::XeeInterpreterError(err) => write!(f, "Xee interpreter error: {}", err),
            Error::DocumentsError(err) => write!(f, "Documents error: {}", err),
        }
    }
}

#[derive(Debug)]
pub struct FieldExtractionError {
    pub field: &'static str,
    pub xpath: &'static str,
    pub extract_id: Option<&'static str>,
    pub source: Box<dyn std::error::Error + Send + Sync>,
}

#[derive(Debug)]
pub struct NoValueFoundError {}

impl std::fmt::Display for NoValueFoundError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "No value found. Make sure the xpath target exists in the XML document."
        )
    }
}

impl std::error::Error for NoValueFoundError {}

impl std::fmt::Display for FieldExtractionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.extract_id {
            Some(id) => write!(
                f,
                "Error extracting field '{}' with XPath '{}' using extract '{}': {}",
                self.field, self.xpath, id, self.source
            ),
            None => write!(
                f,
                "Error extracting field '{}' with XPath '{}': {}",
                self.field, self.xpath, self.source
            ),
        }?;
        if let Some(e) = self
            .source
            .downcast_ref::<xee_interpreter::error::SpannedError>()
        {
            write!(f, " (Xpath {}) \n{}", e.error.message(), e.error.note())?;
        }
        Ok(())
    }
}

impl std::error::Error for FieldExtractionError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(self.source.as_ref())
    }
}

/// A user-friendly error type that provides pretty error messages with XML context
#[derive(Debug)]
pub struct ExtractError {
    /// The underlying core error
    pub error: Error,
    /// Optional span information for highlighting the error location
    pub span: Option<core::ops::Range<usize>>,
    // The lines that the error occurred on
    pub lines: Option<core::ops::Range<usize>>,
    /// Additional context about where the error occurred
    pub context: Option<String>,
}

impl ExtractError {
    /// Create a new ExtractorError from a core Error without span information
    pub fn no_span(error: Error) -> Self {
        Self {
            error,
            span: None,
            lines: None,
            context: None,
        }
    }

    /// Create a new ExtractorError from a core Error
    pub fn new(error: Error, xml: &str) -> Self {
        // Extract the span from the error
        let span: Option<std::ops::Range<usize>> = match &error {
            Error::SpannedError(ref e) => e.span.map(|s| s.range()),
            Error::DocumentsError(ref e) => match e {
                xee_xpath::error::DocumentsError::Parse(ref e) => Some(e.span().range()),
                _ => None,
            },
            _ => None,
        };

        let mut lines = None;
        if let Some(ref span) = span {
            lines = Self::extract_lines(xml, span);
        }

        let mut context = None;
        if let Some(ref span) = span {
            context = Self::extract_xml_context(xml, span);
        }

        Self {
            error,
            span,
            lines,
            context,
        }
    }

    /// Extract the XML snippet around the error location if span is available
    pub fn extract_xml_context(xml: &str, span: &core::ops::Range<usize>) -> Option<String> {
        // Find the start and end of the XML element containing the error
        let xml_bytes = xml.as_bytes();

        // Go forward and backwards 100 characters from the span
        let start = span.start.saturating_sub(100);
        let end = span.end.saturating_add(100);

        // Clamp the start and end to the length of the XML
        let start = start.clamp(0, xml_bytes.len());
        let end = end.clamp(0, xml_bytes.len());

        // Extract the XML snippet
        if start < end && end <= xml_bytes.len() {
            let snippet = &xml_bytes[start..end];
            let snippet_str = String::from_utf8_lossy(snippet).to_string();
            snippet_str.into()
        } else {
            None
        }
    }

    /// Extract the lines that the error occurred on - zero-indexed
    pub fn extract_lines(
        xml: &str,
        span: &core::ops::Range<usize>,
    ) -> Option<core::ops::Range<usize>> {
        let mut byte_pos = 0;
        let mut start_line = None;
        let mut end_line = None;

        for (i, line) in xml.lines().enumerate() {
            let line_len = line.len() + 1; // +1 for '\n'
            let next_byte_pos = byte_pos + line_len;

            if start_line.is_none() && span.start < next_byte_pos {
                start_line = Some(i);
            }

            if span.end <= next_byte_pos {
                end_line = Some(i + 1);
                break;
            }

            byte_pos = next_byte_pos;
        }

        match (start_line, end_line) {
            (Some(start), Some(end)) => Some(start..end),
            _ => None,
        }
    }

    /// Generate a full error message
    pub fn message(&self) -> String {
        let mut message = String::new();

        // Add the main error message
        match &self.error {
            Error::InvalidXPath(msg) => {
                message.push_str(&format!("Invalid XPath expression: {}\n", msg));
            }
            Error::DeserializationError(msg) => {
                message.push_str(&format!("Deserialization error: {}\n", msg));
            }
            Error::UnknownExtractId(msg) => {
                message.push_str(&format!("Unknown extract named: {}\n", msg));
            }
            Error::FieldExtract(err) => {
                message.push_str(&format!("Field extraction error: {}\n", err));
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

        // Add lines if available
        if let Some(lines) = &self.lines {
            if lines.start == lines.end {
                message.push_str(&format!("Line: {}\n", lines.start + 1));
            } else {
                message.push_str(&format!("Lines: {}-{}\n", lines.start + 1, lines.end + 1));
            }
        }

        // Add context if available
        if let Some(context) = &self.context {
            message.push_str(&format!("Context: {}\n", context));
        }

        message
    }
}

impl std::error::Error for ExtractError {}

impl std::fmt::Display for ExtractError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message())
    }
}
