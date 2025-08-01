use xee_extract::{XeeExtract, Extractor, Error};

#[derive(XeeExtract, Debug, PartialEq)]
struct Library {
    #[xpath("//library/@name")]
    name: String,

    #[extract("//library/books/book")]
    books: Vec<Book>,
}

#[derive(XeeExtract, Debug, PartialEq)]
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

#[derive(XeeExtract, Debug, PartialEq)]
struct SimpleStruct {
    #[xpath("//id/text()")]
    id: String,

    #[xpath("//title/text()")]
    title: String,

    #[extract("//nested")]
    nested: NestedStruct,
}

#[derive(XeeExtract, Debug, PartialEq)]
struct NestedStruct {
    #[xpath("value/text()")]
    value: String,

    #[xpath("optional/text()")]
    optional: Option<String>,
}

#[test]
fn test_xpath_and_extract_attributes() {
    let xml = r#"
        <library name="My Library">
            <books>
                <book>
                    <title>The Rust Programming Language</title>
                    <author>Steve Klabnik</author>
                    <year>2018</year>
                    <genre>Programming</genre>
                    <genre>Reference</genre>
                </book>
                <book>
                    <title>Programming Rust</title>
                    <author>Jim Blandy</author>
                    <year>2021</year>
                    <genre>Programming</genre>
                </book>
            </books>
        </library>
    "#;
    
    let extractor = Extractor::new();
    let result: Library = extractor.extract_one(xml).unwrap();
    
    assert_eq!(result.name, "My Library");
    assert_eq!(result.books.len(), 2);
    
    let first_book = &result.books[0];
    assert_eq!(first_book.title, "The Rust Programming Language");
    assert_eq!(first_book.author, "Steve Klabnik");
    assert_eq!(first_book.year, Some(2018));
    assert_eq!(first_book.genres, vec!["Programming", "Reference"]);
    
    let second_book = &result.books[1];
    assert_eq!(second_book.title, "Programming Rust");
    assert_eq!(second_book.author, "Jim Blandy");
    assert_eq!(second_book.year, Some(2021));
    assert_eq!(second_book.genres, vec!["Programming"]);
}

#[test]
fn test_nested_extraction_with_new_attributes() {
    let xml = r#"
        <root>
            <id>123</id>
            <title>Test Title</title>
            <nested>
                <value>Nested Value</value>
                <optional>Optional Value</optional>
            </nested>
        </root>
    "#;
    
    let extractor = Extractor::new();
    let result: SimpleStruct = extractor.extract_one(xml).unwrap();
    
    assert_eq!(result.id, "123");
    assert_eq!(result.title, "Test Title");
    assert_eq!(result.nested.value, "Nested Value");
    assert_eq!(result.nested.optional, Some("Optional Value".to_string()));
}

#[test]
fn test_nested_extraction_with_missing_optional() {
    let xml = r#"
        <root>
            <id>123</id>
            <title>Test Title</title>
            <nested>
                <value>Nested Value</value>
            </nested>
        </root>
    "#;
    
    let extractor = Extractor::new();
    let result: SimpleStruct = extractor.extract_one(xml).unwrap();
    
    assert_eq!(result.id, "123");
    assert_eq!(result.title, "Test Title");
    assert_eq!(result.nested.value, "Nested Value");
    assert_eq!(result.nested.optional, None);
}

#[test]
fn test_missing_required_field_error() {
    let xml = r#"
        <root>
            <id>123</id>
            <!-- Missing title -->
        </root>
    "#;
    
    let extractor = Extractor::new();
    let result: Result<SimpleStruct, Error> = extractor.extract_one(xml);
    
    assert!(result.is_err());
}

#[test]
fn test_empty_vector_extraction() {
    let xml = r#"
        <library name="Empty Library">
            <books>
                <!-- No books -->
            </books>
        </library>
    "#;
    
    let extractor = Extractor::new();
    let result: Library = extractor.extract_one(xml).unwrap();
    
    assert_eq!(result.name, "Empty Library");
    assert_eq!(result.books.len(), 0);
} 