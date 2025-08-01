use xee_extract::{Extractor, Extract};

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
struct ConfigStruct {
    #[xpath("//config/api/base_url/text()")]
    base_url: String,

    #[xpath("//config/api/version/text()")]
    version: String,

    #[xpath("//config/user/id/text()")]
    user_id: String,

    #[xpath("//config/user/name/text()")]
    user_name: String,
}

// Note: Variables are not yet supported in the current implementation
// These tests are placeholders for when variable support is added

#[test]
fn test_extractor_creation() {
    let extractor = Extractor::new();
    assert_eq!(extractor.variables.len(), 0);
}

#[test]
fn test_basic_extraction_without_variables() {
    let xml = r#"
        <entry>
            <id>123</id>
            <title>Test Title</title>
        </entry>
    "#;

    let extractor = Extractor::new();
    let result: SimpleStruct = extractor.extract_one(xml).unwrap();
    assert_eq!(result.id, "123");
    assert_eq!(result.title, "Test Title");
} 