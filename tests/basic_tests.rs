use xee_extract::{XeeExtract, Extractor, XeeExtractMarker, is_xee_extract, downcast_xee_extract, Error};

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

#[test]
fn test_marker_trait_functionality() {
    let xml = r#"
        <entry>
            <id>123</id>
            <title>Sample Title</title>
            <category term="test"/>
        </entry>
    "#;
    
    let extractor = Extractor::new();
    let result: SimpleStruct = extractor.extract_one(xml).unwrap();
    
    // Test that the marker trait works
    let marker_ref: &dyn XeeExtractMarker = &result;
    
    // Test is_xee_extract function
    assert!(is_xee_extract::<SimpleStruct>(marker_ref));
    assert!(!is_xee_extract::<ComplexStruct>(marker_ref));
    
    // Test downcast_xee_extract function
    let downcasted = downcast_xee_extract::<SimpleStruct>(marker_ref);
    assert!(downcasted.is_some());
    let downcasted = downcasted.unwrap();
    assert_eq!(downcasted.id, "123");
    assert_eq!(downcasted.title, "Sample Title");
    assert_eq!(downcasted.category, Some("test".to_string()));
    
    // Test that downcasting to wrong type returns None
    let wrong_downcast = downcast_xee_extract::<ComplexStruct>(marker_ref);
    assert!(wrong_downcast.is_none());
}