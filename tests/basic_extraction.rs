use xee_extract::{Extract, Extractor};

#[derive(Extract, Debug, PartialEq)]
struct SimpleStruct {
    #[xee(xpath("//id/text()"))]
    id: String,

    #[xee(xpath("//title/text()"))]
    title: String,

    #[xee(xpath("//category/@term"))]
    category: Option<String>,
}

#[derive(Extract, Debug, PartialEq)]
struct ComplexStruct {
    #[xee(xpath("//id/text()"))]
    id: String,

    #[xee(xpath("//title/text()"))]
    title: String,

    #[xee(xpath("//subtitle/text()"))]
    subtitle: Option<String>,

    #[xee(xpath("//category/@term"))]
    category: Option<String>,

    #[xee(xpath("//tags/tag/text()"))]
    tags: Vec<String>,
}

#[derive(Extract, Debug, PartialEq)]
struct NestedStruct {
    #[xee(xpath("//id/text()"))]
    id: String,

    #[xee(xpath("//author/name/text()"))]
    author_name: String,

    #[xee(xpath("//author/email/text()"))]
    author_email: Option<String>,
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
    let result: SimpleStruct = extractor.extract_from_str(xml).unwrap();

    assert_eq!(result.id, "123");
    assert_eq!(result.title, "Sample Title");
    assert_eq!(result.category, Some("test".to_string()));
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
    let result: ComplexStruct = extractor.extract_from_str(xml).unwrap();

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
    let result: NestedStruct = extractor.extract_from_str(xml).unwrap();

    assert_eq!(result.id, "789");
    assert_eq!(result.author_name, "John Doe");
    assert_eq!(result.author_email, Some("john@example.com".to_string()));
}
