use xee_extract::{Extractor, Extract};

// Test 1: Context only
#[derive(Extract, Debug, PartialEq)]
#[xee(context("(//entry)[1]"))]
struct ContextOnly {
    #[xee(xpath("@id"))]
    id: String,

    #[xee(xpath("title/text()"))]
    title: String,

    #[xee(xpath("author/name/text()"))]
    author_name: String,
}

// Test 2: Context with conditional logic
#[derive(Extract, Debug, PartialEq)]
#[xee(context("if (self::book) then . else /book"))]
struct Book {
    #[xee(xpath("@id"))]
    id: String,

    #[xee(xpath("title/text()"))]
    title: String,

    #[xee(xpath("author/text()"))]
    author: String,

    #[xee(xpath("price/text()"))]
    price: f64,

    #[xee(xpath("@genre"))]
    genre: Option<String>,

    #[xee(xpath("tags/tag/text()"))]
    tags: Vec<String>,
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
fn test_book_with_context_conditional() {
    let xml = r#"
        <book id="B001" genre="fiction">
            <title>The Rust Programming Language</title>
            <author>Steve Klabnik</author>
            <price>39.99</price>
            <tags>
                <tag>programming</tag>
                <tag>rust</tag>
                <tag>systems</tag>
            </tags>
        </book>
    "#;

    let extractor = Extractor::new();
    let book: Book = extractor.extract_one(xml).unwrap();

    assert_eq!(book.id, "B001");
    assert_eq!(book.title, "The Rust Programming Language");
    assert_eq!(book.author, "Steve Klabnik");
    assert_eq!(book.price, 39.99);
    assert_eq!(book.genre, Some("fiction".to_string()));
    assert_eq!(book.tags, vec!["programming", "rust", "systems"]);
}

#[test]
fn test_book_without_optional_fields_in_context() {
    let xml = r#"
        <book id="B002">
            <title>Programming Rust</title>
            <author>Jim Blandy</author>
            <price>45.50</price>
        </book>
    "#;

    let extractor = Extractor::new();
    let book: Book = extractor.extract_one(xml).unwrap();

    assert_eq!(book.id, "B002");
    assert_eq!(book.title, "Programming Rust");
    assert_eq!(book.author, "Jim Blandy");
    assert_eq!(book.price, 45.50);
    assert_eq!(book.genre, None);
    assert_eq!(book.tags, Vec::<String>::new());
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
    let result = extractor.extract_one::<ContextOnly>(xml);
    
    // This should fail because the XML doesn't have title and author elements
    assert!(result.is_err());
} 