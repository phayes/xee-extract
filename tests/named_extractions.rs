use xee_extract::{Extract, Extractor};

#[derive(Extract, Debug)]
#[xee(ns(atom = "http://www.w3.org/2005/Atom"))]                   // for default
#[xee(default_ns("http://www.w3.org/2005/Atom"))]
#[xee(ns(nlm = "https://id.nlm.nih.gov/datmm/", "extract1"))]     // for extract1
#[xee(ns(atom = "http://www.w3.org/2005/Atom", "extract2"))]      // required for context in extract2
#[xee(context("//atom:entry", "extract2"))]
struct Entry {
    #[xee(xpath("//atom:id/text()"))]
    #[xee(xpath("//nlm:id/text()", "extract1"))]
    #[xee(xpath("atom:id/text()", "extract2"))]
    id: String,

    #[xee(xpath("//atom:title/text()"))]
    #[xee(xpath("//nlm:title/text()", "extract1"))]
    #[xee(xpath("atom:title/text()", "extract2"))]
    title: String,
    
    #[xee(xpath("//atom:author/atom:name/text()"))]
    #[xee(xpath("//nlm:contrib-group/nlm:contrib/nlm:name/text()", "extract1"))]
    #[xee(xpath("atom:author/atom:name/text()", "extract2"))]
    author: Option<String>,
}
#[test]
fn test_default_atom_extraction() {
    let xml = r#"
        <entry xmlns="http://www.w3.org/2005/Atom">
            <id>urn:uuid:12345678-1234-1234-1234-123456789abc</id>
            <title>Atom Title</title>
            <author>
                <name>Alice</name>
            </author>
        </entry>
        "#;

    let extractor = Extractor::default();
    let entry: Entry = extractor.extract_from_str(xml).unwrap();

    assert_eq!(entry.id, "urn:uuid:12345678-1234-1234-1234-123456789abc");
    assert_eq!(entry.title, "Atom Title");
    assert_eq!(entry.author.as_deref(), Some("Alice"));
}

#[test]
fn test_named_extraction_nlm() {
    let xml = r#"
        <article xmlns:nlm="https://id.nlm.nih.gov/datmm/">
            <nlm:id>abc123</nlm:id>
            <nlm:title>NLM Title</nlm:title>
            <nlm:contrib-group>
                <nlm:contrib>
                    <nlm:name>Bob</nlm:name>
                </nlm:contrib>
            </nlm:contrib-group>
        </article>
        "#;

    let extractor = Extractor::named("extract1");
    let entry: Entry = extractor.extract_from_str(xml).unwrap();

    assert_eq!(entry.id, "abc123");
    assert_eq!(entry.title, "NLM Title");
    assert_eq!(entry.author.as_deref(), Some("Bob"));
}

#[test]
fn test_context_extraction() {
    let xml = r#"
        <feed xmlns:atom="http://www.w3.org/2005/Atom">
            <atom:entry>
                <atom:id>c456</atom:id>
                <atom:title>Context Title</atom:title>
                <atom:author>
                    <atom:name>Carol</atom:name>
                </atom:author>
            </atom:entry>
        </feed>
        "#;

    let extractor = Extractor::named("extract2");
    let entry: Entry = extractor.extract_from_str(xml).unwrap();

    assert_eq!(entry.id, "c456");
    assert_eq!(entry.title, "Context Title");
    assert_eq!(entry.author.as_deref(), Some("Carol"));
}
