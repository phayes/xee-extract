use xee_extract::{Extractor, Extract, ExtractorError};

#[derive(Extract, Debug)]
struct SimpleStruct {
    #[xpath("//id/text()")]
    id: String,

    #[xpath("//title/text()")]
    title: String,

    #[xpath("//category/@term")]
    category: Option<String>,
}

#[derive(Extract, Debug)]
struct Book {
    #[xpath("title/text()")]
    title: String,

    #[xpath("author/text()")]
    author: String,

    #[xpath("year/text()")]
    year: Option<i32>,

    #[xpath("genre/text()")]
    genres: Vec<String>,
}

#[test]
fn test_pretty_error_missing_required_field() {
    let xml = r#"
        <entry>
            <title>Sample Title</title>
            <category term="test"/>
        </entry>
    "#;

    let extractor = Extractor::new();
    let result: Result<SimpleStruct, ExtractorError> = extractor.extract_one_pretty(xml);

    assert!(result.is_err());
    let error = result.unwrap_err();
    
    // The error should contain the XML context
    let message = error.pretty_message();
    println!("Pretty error message:\n{}", message);
    
    // Should contain the XML snippet
    assert!(message.contains("XML document:") || message.contains("Relevant XML context:"));
    assert!(message.contains("<entry>"));
    assert!(message.contains("<title>Sample Title</title>"));
    
    // Should mention the missing field or show XPath error
    assert!(message.contains("id") || message.contains("XPath error:"));
}

#[test]
fn test_pretty_error_invalid_xml() {
    let xml = r#"
        <entry>
            <id>123</id>
            <title>Sample Title</title>
            <unclosed>
        </entry>
    "#;

    let extractor = Extractor::new();
    let result: Result<SimpleStruct, ExtractorError> = extractor.extract_one_pretty(xml);

    assert!(result.is_err());
    let error = result.unwrap_err();
    
    let message = error.pretty_message();
    println!("Pretty error message:\n{}", message);
    
    // Should contain XML context
    assert!(message.contains("XML document:") || message.contains("Relevant XML context:"));
    assert!(message.contains("<unclosed>"));
}

#[test]
fn test_pretty_error_invalid_xpath() {
    let xml = r#"
        <entry>
            <id>123</id>
            <title>Sample Title</title>
        </entry>
    "#;

    // Create a struct with an invalid XPath expression
    #[derive(Extract, Debug)]
    struct InvalidXPathStruct {
        #[xpath("//id/text()")]
        id: String,
        
        // This XPath is invalid - it will cause a parsing error
        #[xpath("invalid xpath expression [")]
        invalid: String,
    }

    let extractor = Extractor::new();
    let result: Result<InvalidXPathStruct, ExtractorError> = extractor.extract_one_pretty(xml);

    assert!(result.is_err());
    let error = result.unwrap_err();
    
    let message = error.pretty_message();
    println!("Pretty error message:\n{}", message);
    
    // Should contain XPath error information
    assert!(message.contains("XPath error:"));
    assert!(message.contains("Relevant XML context:"));
}

#[test]
fn test_pretty_error_deserialization() {
    let xml = r#"
        <book>
            <title>Sample Book</title>
            <author>John Doe</author>
            <year>not_a_number</year>
            <genre>Fiction</genre>
        </book>
    "#;

    let extractor = Extractor::new();
    let result: Result<Book, ExtractorError> = extractor.extract_one_pretty(xml);

    assert!(result.is_err());
    let error = result.unwrap_err();
    
    let message = error.pretty_message();
    println!("Pretty error message:\n{}", message);
    
    // Should contain deserialization error information
    assert!(message.contains("XPath error:") || message.contains("Deserialization error:"));
    assert!(message.contains("XML document:") || message.contains("Relevant XML context:"));
    assert!(message.contains("<year>not_a_number</year>"));
}

#[test]
fn test_pretty_error_with_span() {
    let xml = r#"
        <entry>
            <id>123</id>
            <title>Sample Title</title>
            <malformed>No closing tag
        </entry>
    "#;

    let extractor = Extractor::new();
    let result: Result<SimpleStruct, ExtractorError> = extractor.extract_one_pretty(xml);

    assert!(result.is_err());
    let error = result.unwrap_err();
    
    let message = error.pretty_message();
    println!("Pretty error message:\n{}", message);
    
    // Should contain XML context and line information
    assert!(message.contains("XML document:") || message.contains("Relevant XML context:"));
    assert!(message.contains("Error occurred around line") || message.contains("Error occurred in XML document"));
    assert!(message.contains("<malformed>No closing tag"));
}

#[test]
fn test_pretty_error_empty_xml() {
    let xml = "";

    let extractor = Extractor::new();
    let result: Result<SimpleStruct, ExtractorError> = extractor.extract_one_pretty(xml);

    assert!(result.is_err());
    let error = result.unwrap_err();
    
    let message = error.pretty_message();
    println!("Pretty error message:\n{}", message);
    
    // Should contain document error information
    assert!(message.contains("XML document error:"));
}

#[test]
fn test_pretty_error_with_context() {
    let xml = r#"
        <entry>
            <title>Sample Title</title>
            <!-- Missing required id field -->
        </entry>
    "#;

    let extractor = Extractor::new();
    let result: Result<SimpleStruct, ExtractorError> = extractor.extract_one_pretty(xml);

    assert!(result.is_err());
    let error = result.unwrap_err();
    
    // Add additional context
    let error_with_context = error.with_context("Failed to extract SimpleStruct".to_string());
    
    let message = error_with_context.pretty_message();
    println!("Pretty error message with context:\n{}", message);
    
    // Should contain the additional context
    assert!(message.contains("Context: Failed to extract SimpleStruct"));
    assert!(message.contains("XML document:") || message.contains("Relevant XML context:"));
} 