use xee_extract::{Error, Extractor, Extract};

#[derive(Extract, Debug, PartialEq)]
struct SimpleStruct {
    #[xpath("//id/text()")]
    id: String,

    #[xpath("//title/text()")]
    title: String,

    #[xpath("//category/@term")]
    category: Option<String>,
}

#[derive(Extract, Debug, PartialEq)]
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

#[derive(Extract, Debug, PartialEq)]
struct SimpleStructWithNested {
    #[xpath("//id/text()")]
    id: String,

    #[xpath("//title/text()")]
    title: String,

    #[extract("//nested")]
    nested: NestedStruct,
}

#[derive(Extract, Debug, PartialEq)]
struct NestedStruct {
    #[xpath("value/text()")]
    value: String,

    #[xpath("optional/text()")]
    optional: Option<String>,
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
fn test_missing_required_field_error_with_nested() {
    let xml = r#"
        <root>
            <id>123</id>
            <!-- Missing title -->
        </root>
    "#;

    let extractor = Extractor::new();
    let result: Result<SimpleStructWithNested, Error> = extractor.extract_one(xml);

    assert!(result.is_err());
}

#[test]
fn test_missing_nested_struct_error() {
    let xml = r#"
        <root>
            <id>123</id>
            <title>Test Title</title>
            <!-- Missing nested element -->
        </root>
    "#;

    let extractor = Extractor::new();
    let result: Result<SimpleStructWithNested, Error> = extractor.extract_one(xml);

    assert!(result.is_err());
}

#[test]
fn test_missing_required_field_in_nested_struct() {
    let xml = r#"
        <root>
            <id>123</id>
            <title>Test Title</title>
            <nested>
                <!-- Missing required value field -->
                <optional>Optional Value</optional>
            </nested>
        </root>
    "#;

    let extractor = Extractor::new();
    let result: Result<SimpleStructWithNested, Error> = extractor.extract_one(xml);

    assert!(result.is_err());
}

#[test]
fn test_invalid_xpath_expression() {
    let xml = r#"
        <entry>
            <id>123</id>
            <title>Sample Title</title>
        </entry>
    "#;

    let extractor = Extractor::new();
    // This would fail if we had an invalid XPath expression
    let result: Result<SimpleStruct, Error> = extractor.extract_one(xml);

    // Should succeed with valid XML and XPath
    assert!(result.is_ok());
}

#[test]
fn test_empty_xml_document() {
    let xml = "";

    let extractor = Extractor::new();
    let result: Result<SimpleStruct, Error> = extractor.extract_one(xml);

    assert!(result.is_err());
}

#[test]
fn test_xml_with_only_whitespace() {
    let xml = "   \n   \t   ";

    let extractor = Extractor::new();
    let result: Result<SimpleStruct, Error> = extractor.extract_one(xml);

    assert!(result.is_err());
}

#[test]
fn test_xml_with_malformed_tags() {
    let xml = r#"
        <entry>
            <id>123</id>
            <title>Sample Title</title>
            <malformed>No closing tag
        </entry>
    "#;

    let extractor = Extractor::new();
    let result: Result<SimpleStruct, Error> = extractor.extract_one(xml);

    assert!(result.is_err());
}

#[test]
fn test_xml_with_invalid_characters() {
    let xml = r#"
        <entry>
            <id>123</id>
            <title>Sample Title with invalid chars: <>&"</title>
        </entry>
    "#;

    let extractor = Extractor::new();
    let result: Result<SimpleStruct, Error> = extractor.extract_one(xml);

    // This should fail due to invalid XML characters
    assert!(result.is_err());
} 