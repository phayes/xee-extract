use xee_extract::{Extractor, Extract};

// Test 1: Basic namespace prefixes only
#[derive(Extract, Debug, PartialEq)]
#[ns(
    atom = "http://www.w3.org/2005/Atom",
    dc = "http://purl.org/dc/elements/1.1/"
)]
#[context("(//entry)[1]")]
struct NamespaceOnly {
    #[xpath("atom:id/text()")]
    id: String,

    #[xpath("atom:title/text()")]
    title: String,

    #[xpath("dc:creator/text()")]
    creator: Option<String>,
}

// Test 2: Default namespace only
#[derive(Extract, Debug, PartialEq)]
#[default_ns("http://www.w3.org/2005/Atom")]
#[context("(//entry)[1]")]
struct DefaultNamespaceOnly {
    #[xpath("id/text()")]
    id: String,

    #[xpath("title/text()")]
    title: String,

    #[xpath("author/name/text()")]
    author_name: String,
}

// Test 3: Context only
#[derive(Extract, Debug, PartialEq)]
#[context("(//entry)[1]")]
struct ContextOnly {
    #[xpath("@id")]
    id: String,

    #[xpath("title/text()")]
    title: String,

    #[xpath("author/name/text()")]
    author_name: String,
}

// Test 4: Namespace prefixes + Context
#[derive(Extract, Debug, PartialEq)]
#[ns(
    atom = "http://www.w3.org/2005/Atom",
    dc = "http://purl.org/dc/elements/1.1/"
)]
#[context("(//entry)[1]")]
struct NamespaceAndContext {
    #[xpath("atom:id/text()")]
    id: String,

    #[xpath("atom:title/text()")]
    title: String,

    #[xpath("dc:creator/text()")]
    creator: Option<String>,
}

// Test 5: Default namespace + Context
#[derive(Extract, Debug, PartialEq)]
#[default_ns("http://www.w3.org/2005/Atom")]
#[context("(//entry)[1]")]
struct DefaultNamespaceAndContext {
    #[xpath("id/text()")]
    id: String,

    #[xpath("title/text()")]
    title: String,

    #[xpath("author/name/text()")]
    author_name: String,
}

// Test 6: Default namespace + Namespace prefixes
#[derive(Extract, Debug, PartialEq)]
#[default_ns("http://www.w3.org/2005/Atom")]
#[ns(
    dc = "http://purl.org/dc/elements/1.1/"
)]
#[context("(//entry)[1]")]
struct DefaultAndPrefixedNamespaces {
    #[xpath("id/text()")]
    id: String,

    #[xpath("title/text()")]
    title: String,

    #[xpath("dc:creator/text()")]
    creator: Option<String>,
}

// Test 7: All three combined
#[derive(Extract, Debug, PartialEq)]
#[default_ns("http://www.w3.org/2005/Atom")]
#[ns(
    dc = "http://purl.org/dc/elements/1.1/"
)]
#[context("(//entry)[1]")]
struct AllCombined {
    #[xpath("id/text()")]
    id: String,

    #[xpath("title/text()")]
    title: String,

    #[xpath("dc:creator/text()")]
    creator: Option<String>,
}

// Test 8: Nested structs with different namespace configurations
#[derive(Extract, Debug, PartialEq)]
#[default_ns("http://www.w3.org/2005/Atom")]
#[ns(
    dc = "http://purl.org/dc/elements/1.1/"
)]
#[context("(//entry)[1]")]
struct ParentWithNamespaces {
    #[xpath("id/text()")]
    id: String,

    #[extract("author")]
    author: ChildWithDifferentNamespaces,
}

#[derive(Extract, Debug, PartialEq)]
#[ns(
    atom = "http://www.w3.org/2005/Atom"
)]
struct ChildWithDifferentNamespaces {
    #[xpath("atom:name/text()")]
    name: String,

    #[xpath("atom:email/text()")]
    email: Option<String>,
}

// Test 9: Multiple default namespaces in nested structs
#[derive(Extract, Debug, PartialEq)]
#[default_ns("http://www.w3.org/2005/Atom")]
#[context("(//entry)[1]")]
struct ParentWithDefaultNamespace {
    #[xpath("id/text()")]
    id: String,

    #[extract("author")]
    author: ChildWithDefaultNamespace,
}

#[derive(Extract, Debug, PartialEq)]
#[default_ns("http://www.w3.org/2005/Atom")]
struct ChildWithDefaultNamespace {
    #[xpath("name/text()")]
    name: String,

    #[xpath("email/text()")]
    email: Option<String>,
}

#[test]
fn test_namespace_only() {
    let xml = r#"
        <feed xmlns:atom="http://www.w3.org/2005/Atom" 
              xmlns:dc="http://purl.org/dc/elements/1.1/">
            <entry>
                <atom:id>123</atom:id>
                <atom:title>Test Title</atom:title>
                <dc:creator>Test Creator</dc:creator>
            </entry>
        </feed>
    "#;

    let extractor = Extractor::new();
    let result: NamespaceOnly = extractor.extract_one(xml).unwrap();

    assert_eq!(result.id, "123");
    assert_eq!(result.title, "Test Title");
    assert_eq!(result.creator, Some("Test Creator".to_string()));
}

#[test]
fn test_default_namespace_only() {
    let xml = r#"
        <feed xmlns="http://www.w3.org/2005/Atom">
            <entry>
                <id>123</id>
                <title>Test Title</title>
                <author>
                    <name>Test Author</name>
                </author>
            </entry>
        </feed>
    "#;

    let extractor = Extractor::new();
    let result: DefaultNamespaceOnly = extractor.extract_one(xml).unwrap();

    assert_eq!(result.id, "123");
    assert_eq!(result.title, "Test Title");
    assert_eq!(result.author_name, "Test Author");
}

#[test]
fn test_context_only() {
    let xml = r#"
        <feed>
            <entry id="123">
                <title>Test Title</title>
                <author>
                    <name>Test Author</name>
                </author>
            </entry>
        </feed>
    "#;

    let extractor = Extractor::new();
    let result: ContextOnly = extractor.extract_one(xml).unwrap();

    assert_eq!(result.id, "123");
    assert_eq!(result.title, "Test Title");
    assert_eq!(result.author_name, "Test Author");
}

#[test]
fn test_namespace_and_context() {
    let xml = r#"
        <feed xmlns:atom="http://www.w3.org/2005/Atom" 
              xmlns:dc="http://purl.org/dc/elements/1.1/">
            <entry>
                <atom:id>123</atom:id>
                <atom:title>Test Title</atom:title>
                <dc:creator>Test Creator</dc:creator>
            </entry>
        </feed>
    "#;

    let extractor = Extractor::new();
    let result: NamespaceAndContext = extractor.extract_one(xml).unwrap();

    assert_eq!(result.id, "123");
    assert_eq!(result.title, "Test Title");
    assert_eq!(result.creator, Some("Test Creator".to_string()));
}

#[test]
fn test_default_namespace_and_context() {
    let xml = r#"
        <feed xmlns="http://www.w3.org/2005/Atom">
            <entry>
                <id>123</id>
                <title>Test Title</title>
                <author>
                    <name>Test Author</name>
                </author>
            </entry>
        </feed>
    "#;

    let extractor = Extractor::new();
    let result: DefaultNamespaceAndContext = extractor.extract_one(xml).unwrap();

    assert_eq!(result.id, "123");
    assert_eq!(result.title, "Test Title");
    assert_eq!(result.author_name, "Test Author");
}

#[test]
fn test_default_and_prefixed_namespaces() {
    let xml = r#"
        <feed xmlns="http://www.w3.org/2005/Atom" 
              xmlns:dc="http://purl.org/dc/elements/1.1/">
            <entry>
                <id>123</id>
                <title>Test Title</title>
                <dc:creator>Test Creator</dc:creator>
            </entry>
        </feed>
    "#;

    let extractor = Extractor::new();
    let result: DefaultAndPrefixedNamespaces = extractor.extract_one(xml).unwrap();

    assert_eq!(result.id, "123");
    assert_eq!(result.title, "Test Title");
    assert_eq!(result.creator, Some("Test Creator".to_string()));
}

#[test]
fn test_all_combined() {
    let xml = r#"
        <feed xmlns="http://www.w3.org/2005/Atom" 
              xmlns:dc="http://purl.org/dc/elements/1.1/">
            <entry>
                <id>123</id>
                <title>Test Title</title>
                <dc:creator>Test Creator</dc:creator>
            </entry>
        </feed>
    "#;

    let extractor = Extractor::new();
    let result: AllCombined = extractor.extract_one(xml).unwrap();

    assert_eq!(result.id, "123");
    assert_eq!(result.title, "Test Title");
    assert_eq!(result.creator, Some("Test Creator".to_string()));
}

#[test]
fn test_nested_structs_with_different_namespaces() {
    let xml = r#"
        <feed xmlns="http://www.w3.org/2005/Atom" 
              xmlns:dc="http://purl.org/dc/elements/1.1/"
              xmlns:atom="http://www.w3.org/2005/Atom">
            <entry>
                <id>123</id>
                <atom:author>
                    <atom:name>Test Author</atom:name>
                    <atom:email>test@example.com</atom:email>
                </atom:author>
            </entry>
        </feed>
    "#;

    let extractor = Extractor::new();
    let result: ParentWithNamespaces = extractor.extract_one(xml).unwrap();

    assert_eq!(result.id, "123");
    assert_eq!(result.author.name, "Test Author");
    assert_eq!(result.author.email, Some("test@example.com".to_string()));
}

#[test]
fn test_nested_structs_with_default_namespaces() {
    let xml = r#"
        <feed xmlns="http://www.w3.org/2005/Atom">
            <entry>
                <id>123</id>
                <author>
                    <name>Test Author</name>
                    <email>test@example.com</email>
                </author>
            </entry>
        </feed>
    "#;

    let extractor = Extractor::new();
    let result: ParentWithDefaultNamespace = extractor.extract_one(xml).unwrap();

    assert_eq!(result.id, "123");
    assert_eq!(result.author.name, "Test Author");
    assert_eq!(result.author.email, Some("test@example.com".to_string()));
}

#[test]
fn test_error_handling_invalid_xpath() {
    let xml = r#"
        <feed xmlns="http://www.w3.org/2005/Atom">
            <entry>
                <id>123</id>
            </entry>
        </feed>
    "#;

    let extractor = Extractor::new();
    let result = extractor.extract_one::<DefaultNamespaceOnly>(xml);
    
    // This should fail because the XML doesn't have title and author elements
    assert!(result.is_err());
}

#[test]
fn test_error_handling_missing_namespace() {
    let xml = r#"
        <feed>
            <entry>
                <id>123</id>
                <title>Test Title</title>
            </entry>
        </feed>
    "#;

    let extractor = Extractor::new();
    let result = extractor.extract_one::<DefaultNamespaceOnly>(xml);
    
    // This should fail because the XML doesn't have the default namespace
    assert!(result.is_err());
} 