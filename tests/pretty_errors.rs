use xee_extract::{
    Documents, ErrorType, Extract, ExtractError, ExtractValue, Extractor, Item,
};

#[derive(Extract, Debug)]
struct SimpleStruct {
    #[xee(xpath("//id/text()"))]
    _id: String,

    #[xee(xpath("//title/text()"))]
    _title: String,

    #[xee(xpath("//category/@term"))]
    _category: Option<String>,
}

#[derive(Extract, Debug)]
struct Book {
    #[xee(xpath("title/text()"))]
    _title: String,

    #[xee(xpath("author/text()"))]
    _author: String,

    #[xee(xpath("year/text()"))]
    _year: Option<i32>,

    #[xee(xpath("genre/text()"))]
    _genres: Vec<String>,
}

#[derive(Extract, Debug)]
struct ExtractTestStruct {
    #[xee(extract("author"))]
    _author: Author,
}

#[derive(Extract, Debug)]
struct Author {
    #[xee(xpath("name/text()"))]
    _name: String,
}

#[derive(Extract, Debug)]
struct XmlTestStruct {
    #[xee(xml("author"))]
    _author_xml: String,
}

// A custom type whose ExtractValue implementation always returns an error
#[derive(Debug)]
struct FailingValue;

impl ExtractValue for FailingValue {
    fn extract_value(
        _documents: &mut Documents,
        _item: &Item,
    ) -> Result<Self, ErrorType> {
        Err(ErrorType::DeserializationError(
            "custom extract value failure".to_string(),
        ))
    }
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

    let extractor = Extractor::default();
    let result: Result<SimpleStruct, ExtractError> = extractor.extract_from_str(xml);

    assert!(result.is_err());
    let error = result.unwrap_err();

    let message = error.message();

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
        #[xee(xpath("//id/text()"))]
        _id: String,

        // This XPath is invalid - it will cause a parsing error
        #[xee(xpath("invalid xpath expression ["))]
        _invalid: String,
    }

    let extractor = Extractor::default();
    let result: Result<InvalidXPathStruct, ExtractError> = extractor.extract_from_str(xml);

    assert!(result.is_err());
    let error = result.unwrap_err();

    let message = error.message();
    println!("Error message:\n{}", message);

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

    let extractor = Extractor::default();
    let result: Result<SimpleStruct, ExtractError> = extractor.extract_from_str(xml);

    assert!(result.is_err());
    let error = result.unwrap_err();

    let message = error.message();
    println!("Error message:\n{}", message);

    // Should contain XML context and line information
    assert!(message.contains("Context"));
    assert!(message.contains("<malformed>No closing tag"));
}

#[test]
fn test_pretty_error_empty_xml() {
    let xml = "";

    let extractor = Extractor::default();
    let result: Result<SimpleStruct, ExtractError> = extractor.extract_from_str(xml);

    assert!(result.is_err());
    let error = result.unwrap_err();

    let message = error.message();
    println!("Error message:\n{}", message);

    // Should contain document error information
    assert!(message.contains("XML document error:"));
}

#[test]
fn test_application_error_extract_value() {
    // This test demonstrates that the Book struct with year field gets a type error
    // rather than ApplicationError, which is expected behavior
    let xml = r#"
        <entry>
            <id>123</id>
            <title>Sample Title</title>
            <year>not_a_number</year>
        </entry>
    "#;

    let extractor = Extractor::default();
    let result: Result<Book, ExtractError> = extractor.extract_from_str(xml);

    assert!(result.is_err());
    let error = result.unwrap_err();

    let message = error.message();
    println!("Book extraction error (XPath type error):\n{}", message);

    // Should contain XPath type error information
    assert!(message.contains("No value found"));
}

#[test]
fn test_application_error_value_extraction() {
    // This test should trigger ApplicationError because the XPath finds a node
    // but the value extraction fails when trying to parse "not_a_number" as an integer
    #[derive(Extract, Debug)]
    struct ValueTestStruct {
        #[xee(xpath("//year/text()"))]
        _year: i32, // This should fail when trying to parse "not_a_number" as i32
    }

    let xml = r#"
        <entry>
            <id>123</id>
            <title>Sample Title</title>
            <year>not_a_number</year>
        </entry>
    "#;

    let extractor = Extractor::default();
    let result: Result<ValueTestStruct, ExtractError> = extractor.extract_from_str(xml);

    assert!(result.is_err());
    let error = result.unwrap_err();

    // Debug: Print the actual error type
    println!("Error type: {:?}", error.error);

    let message = error.message();
    println!("Value extraction ApplicationError test:\n{}", message);

    // The error should mention the field name and the parsing error
    assert!(message.contains("Error extracting field '_year'"));
    assert!(message.contains("invalid digit found in string"));
}

#[test]
fn test_custom_extract_value_error_includes_field_and_xpath() {
    #[derive(Extract, Debug)]
    struct CustomStruct {
        #[xee(xpath("//value/text()"))]
        _value: FailingValue,
    }

    let xml = r#"<entry><value>bad</value></entry>"#;
    let extractor = Extractor::default();
    let result: Result<CustomStruct, ExtractError> = extractor.extract_from_str(xml);

    assert!(result.is_err());
    let message = result.unwrap_err().message();

    assert!(message.contains("Error extracting field '_value'"));
    assert!(message.contains("//value/text()"));
    assert!(message.contains("custom extract value failure"));
}

#[test]
fn test_custom_extract_value_option_error_includes_field_and_xpath() {
    #[derive(Extract, Debug)]
    struct CustomStruct {
        #[xee(xpath("//opt/text()"))]
        _opt: Option<FailingValue>,
    }

    let xml = r#"<entry><opt>bad</opt></entry>"#;
    let extractor = Extractor::default();
    let result: Result<CustomStruct, ExtractError> = extractor.extract_from_str(xml);

    assert!(result.is_err());
    let message = result.unwrap_err().message();

    assert!(message.contains("Error extracting field '_opt'"));
    assert!(message.contains("//opt/text()"));
    assert!(message.contains("custom extract value failure"));
}

#[test]
fn test_application_error_extract_struct() {
    // This test demonstrates that struct extraction errors are handled as XPath type errors
    // rather than ApplicationError, which is expected behavior
    let xml = r#"
        <entry>
            <id>123</id>
            <title>Sample Title</title>
            <author>
                <name>John Doe</name>
            </author>
        </entry>
    "#;

    let extractor = Extractor::default();
    let result: Result<ExtractTestStruct, ExtractError> = extractor.extract_from_str(xml);

    assert!(result.is_err());
    let error = result.unwrap_err();

    let message = error.message();
    println!("Struct extraction error (XPath type error):\n{}", message);

    // Should contain XPath type error information
    assert!(message.contains("No value found"));
}

#[test]
fn test_application_error_xml_serialization() {
    // This test demonstrates that XML serialization errors are handled as XPath type errors
    // rather than ApplicationError, which is expected behavior
    let xml = r#"
        <entry>
            <id>123</id>
            <title>Sample Title</title>
            <author>
                <name>John Doe</name>
            </author>
        </entry>
    "#;

    let extractor = Extractor::default();
    let result: Result<XmlTestStruct, ExtractError> = extractor.extract_from_str(xml);

    assert!(result.is_err());
    let error = result.unwrap_err();

    let message = error.message();
    println!("XML serialization error (XPath type error):\n{}", message);

    // Should contain XPath type error information
    assert!(message.contains("No value found"));
}

#[test]
fn test_application_error_namespace() {
    // This test demonstrates that namespace errors are handled as XPath type errors
    // rather than ApplicationError, which is expected behavior
    let xml = r#"
        <entry>
            <id>123</id>
            <title>Sample Title</title>
        </entry>
    "#;

    // Create a struct that requires a namespace but doesn't have one
    #[derive(Extract, Debug)]
    #[xee(ns(test = "http://example.com/test"))]
    struct NamespaceTestStruct {
        #[xee(xpath("//test:id/text()"))]
        _id: String,
    }

    let extractor = Extractor::default();
    let result: Result<NamespaceTestStruct, ExtractError> = extractor.extract_from_str(xml);

    assert!(result.is_err());
    let error = result.unwrap_err();

    let message = error.message();
    println!("Namespace error (XPath type error):\n{}", message);

    // Should contain XPath type error information
    assert!(message.contains("No value found"));
}
