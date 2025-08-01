use xee_extract::{Extractor, Extract};

#[derive(Extract, Debug, PartialEq)]
struct DocumentWithXml {
    #[xpath("//id/text()")]
    id: String,

    #[xpath("//title/text()")]
    title: String,

    #[xml("//content")]
    content: String,

    #[xml("//metadata")]
    metadata: Option<String>,
}

#[derive(Extract, Debug, PartialEq)]
struct ComplexDocument {
    #[xpath("//id/text()")]
    id: String,

    #[xml("//section")]
    sections: Vec<String>,
}

#[derive(Extract, Debug, PartialEq)]
struct XmlWithAttributes {
    #[xpath("//id/text()")]
    id: String,

    #[xml("//element[@type='special']")]
    special_content: Option<String>,
}

#[derive(Extract, Debug, PartialEq)]
struct SimpleXmlTest {
    #[xpath("//id/text()")]
    id: String,

    #[xml("//content")]
    content: String,
}

#[test]
fn test_xml_attribute_basic() {
    let xml = r#"
        <document>
            <id>123</id>
            <title>Test Document</title>
            <content>
                <p>This is some content</p>
                <p>With multiple paragraphs</p>
            </content>
            <metadata>
                <author>John Doe</author>
                <date>2023-01-01</date>
            </metadata>
        </document>
    "#;

    let extractor = Extractor::new();
    let result: DocumentWithXml = extractor.extract_one(xml).unwrap();

    assert_eq!(result.id, "123");
    assert_eq!(result.title, "Test Document");
    assert!(result.content.contains("<p>This is some content</p>"));
    assert!(result.content.contains("<p>With multiple paragraphs</p>"));

    let metadata = result.metadata.unwrap();
    assert!(metadata.contains("<author>John Doe</author>"));
    assert!(metadata.contains("<date>2023-01-01</date>"));
}

#[test]
fn test_xml_attribute_with_vectors() {
    let xml = r#"
        <document>
            <id>456</id>
            <section>
                <h1>Section 1</h1>
                <p>Content 1</p>
            </section>
            <section>
                <h1>Section 2</h1>
                <p>Content 2</p>
            </section>
        </document>
    "#;

    let extractor = Extractor::new();
    let result: ComplexDocument = extractor.extract_one(xml).unwrap();

    assert_eq!(result.id, "456");
    assert_eq!(result.sections.len(), 2);
    assert!(result.sections[0].contains("<h1>Section 1</h1>"));
    assert!(result.sections[0].contains("<p>Content 1</p>"));
    assert!(result.sections[1].contains("<h1>Section 2</h1>"));
    assert!(result.sections[1].contains("<p>Content 2</p>"));
}

#[test]
fn test_xml_attribute_with_optional() {
    let xml = r#"
        <document>
            <id>789</id>
            <element type="special">
                <special>This is special content</special>
            </element>
        </document>
    "#;

    let extractor = Extractor::new();
    let result: XmlWithAttributes = extractor.extract_one(xml).unwrap();

    assert_eq!(result.id, "789");
    assert!(result
        .special_content
        .unwrap()
        .contains("<special>This is special content</special>"));
}

#[test]
fn test_xml_attribute_missing_optional() {
    let xml = r#"
        <document>
            <id>789</id>
            <!-- No special element -->
        </document>
    "#;

    let extractor = Extractor::new();
    let result: XmlWithAttributes = extractor.extract_one(xml).unwrap();

    assert_eq!(result.id, "789");
    assert_eq!(result.special_content, None);
}

#[test]
fn test_xml_attribute_empty_vector() {
    let xml = r#"
        <document>
            <id>999</id>
            <!-- No sections -->
        </document>
    "#;

    let extractor = Extractor::new();
    let result: ComplexDocument = extractor.extract_one(xml).unwrap();

    assert_eq!(result.id, "999");
    assert_eq!(result.sections.len(), 0);
}

#[test]
fn test_xml_attribute_debug() {
    let xml = r#"
        <document>
            <id>123</id>
            <title>Test Document</title>
            <content>
                <p>This is some content</p>
            </content>
        </document>
    "#;

    let extractor = Extractor::new();
    let result: DocumentWithXml = extractor.extract_one(xml).unwrap();

    println!("Content extracted: '{}'", result.content);
    println!("Content length: {}", result.content.len());
    println!("Content contains '<p>': {}", result.content.contains("<p>"));
    println!(
        "Content contains 'This is some content': {}",
        result.content.contains("This is some content")
    );
}

#[test]
fn test_xml_attribute_simple() {
    let xml = r#"
        <document>
            <id>123</id>
            <title>Test Document</title>
            <content>
                <p>This is some content</p>
            </content>
        </document>
    "#;

    let extractor = Extractor::new();
    let result: DocumentWithXml = extractor.extract_one(xml).unwrap();

    println!("ID: {}", result.id);
    println!("Content: '{}'", result.content);
}

#[test]
fn test_xml_attribute_debug_simple() {
    let xml = r#"
        <document>
            <id>123</id>
            <title>Test Document</title>
            <content>
                <p>This is some content</p>
            </content>
        </document>
    "#;

    let extractor = Extractor::new();
    let result: DocumentWithXml = extractor.extract_one(xml).unwrap();

    println!("ID: {}", result.id);
    println!("Content: '{}'", result.content);
}

#[test]
fn test_xml_attribute_working_xpath() {
    let xml = r#"
        <document>
            <id>123</id>
            <title>Test Document</title>
            <content>
                <p>This is some content</p>
            </content>
        </document>
    "#;

    let extractor = Extractor::new();
    let result: DocumentWithXml = extractor.extract_one(xml).unwrap();

    println!("ID: {}", result.id);
    println!("Content: '{}'", result.content);
}

#[test]
fn test_xml_attribute_simple_xpath() {
    let xml = r#"
        <document>
            <id>123</id>
            <content>
                <p>This is some content</p>
            </content>
        </document>
    "#;

    let extractor = Extractor::new();
    let result: SimpleXmlTest = extractor.extract_one(xml).unwrap();

    println!("ID: {}", result.id);
    println!("Content: '{}'", result.content);
}
