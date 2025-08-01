use xee_interpreter;
use xee_xpath;

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
    pub fn extract_lines(xml: &str, span: &core::ops::Range<usize>) -> Option<core::ops::Range<usize>> {
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
        write!(f, "{}", self.pretty_message())
    }
} 