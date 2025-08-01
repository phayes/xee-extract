use xee_extract::{Extractor, Extract, ExtractError};

#[derive(Extract, Debug)]
struct SimpleStruct {
    #[xpath("//id/text()")]
    _id: String,

    #[xpath("//title/text()")]
    _title: String,

    #[xpath("//category/@term")]
    _category: Option<String>,
}

#[derive(Extract, Debug)]
struct Book {
    #[xpath("title/text()")]
    _title: String,

    #[xpath("author/text()")]
    _author: String,

    #[xpath("year/text()")]
    _year: Option<i32>,

    #[xpath("genre/text()")]
    _genres: Vec<String>,
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
    let result: Result<SimpleStruct, ExtractError> = extractor.extract_one(xml);

    assert!(result.is_err());
    let error = result.unwrap_err();
    
    let message = error.pretty_message();
    println!("Pretty error message:\n{}", message);
    
    // Should contain XML context
    assert!(message.contains("Context:"));
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
        _id: String,
        
        // This XPath is invalid - it will cause a parsing error
        #[xpath("invalid xpath expression [")]
        _invalid: String,
    }

    let extractor = Extractor::new();
    let result: Result<InvalidXPathStruct, ExtractError> = extractor.extract_one(xml);

    assert!(result.is_err());
    let error = result.unwrap_err();
    
    let message = error.pretty_message();
    println!("Pretty error message:\n{}", message);
    
    // Should contain XPath error information
    assert!(message.contains("XPath error:"));
    assert!(message.contains("Context:"));
}

#[test]
fn test_pretty_error_with_span() {
    let xml: &'static str = r#"
        <entry>
            <id>123</id>
            <title>Sample Title</title>
            <malformed>No closing tag
        </entry>
    "#;

    let extractor = Extractor::new();
    let result: Result<SimpleStruct, ExtractError> = extractor.extract_one(xml);

    assert!(result.is_err());
    let error = result.unwrap_err();

    dbg!(&error);
    
    let message = error.pretty_message();
    println!("Pretty error message:\n{}", message);
    
    // Should contain XML context and line information
    assert!(message.contains("Context"));
    assert!(message.contains("<malformed>No closing tag"));
}

#[test]
fn test_pretty_error_empty_xml() {
    let xml = "";

    let extractor = Extractor::new();
    let result: Result<SimpleStruct, ExtractError> = extractor.extract_one(xml);

    assert!(result.is_err());
    let error = result.unwrap_err();
    
    let message = error.pretty_message();
    println!("Pretty error message:\n{}", message);
    
    // Should contain document error information
    assert!(message.contains("XML document error:"));
}
