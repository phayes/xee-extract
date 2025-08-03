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

#[derive(Extract, Debug, PartialEq)]
struct Book {
    #[xee(xpath("/book/title/text()"))]
    title: String,

    #[xee(xpath("/book/author/text()"))]
    author: String,

    #[xee(xpath("/book/year/text()"))]
    year: Option<i32>,

    #[xee(xpath("/book/genre/text()"))]
    genres: Vec<String>,
}

#[test]
fn test_extraction_with_missing_optional_field() {
    let xml = r#"
        <entry>
            <id>123</id>
            <title>Sample Title</title>
        </entry>
    "#;

    let extractor = Extractor::default();
    let result: SimpleStruct = extractor.extract_from_str(xml).unwrap();

    assert_eq!(result.id, "123");
    assert_eq!(result.title, "Sample Title");
    assert_eq!(result.category, None);
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

    let extractor = Extractor::default();
    let result: NestedStruct = extractor.extract_from_str(xml).unwrap();

    assert_eq!(result.id, "789");
    assert_eq!(result.author_name, "Jane Smith");
    assert_eq!(result.author_email, None);
}

#[test]
fn test_book_without_optional_fields() {
    let xml = r#"
        <book id="B002">
            <title>Programming Rust</title>
            <author>Jim Blandy</author>
            <price>45.50</price>
        </book>
    "#;

    let extractor = Extractor::default();
    let book: Book = extractor.extract_from_str(xml).unwrap();

    assert_eq!(book.title, "Programming Rust");
    assert_eq!(book.author, "Jim Blandy");
    assert_eq!(book.year, None);
    assert_eq!(book.genres, Vec::<String>::new());
}

#[test]
fn test_person_without_optional_fields() {
    let xml = r#"
        <person id="P002">
            <name>
                <first>Jane</first>
                <last>Smith</last>
            </name>
        </person>
    "#;

    let extractor = Extractor::default();
    let person: Person = extractor.extract_from_str(xml).unwrap();

    assert_eq!(person.id, "P002");
    assert_eq!(person.first_name, "Jane");
    assert_eq!(person.last_name, "Smith");
    assert_eq!(person.email, None);
    assert_eq!(person.street, None);
    assert_eq!(person.city, None);
}

#[derive(Extract, Debug, PartialEq)]
struct Person {
    #[xee(xpath("//person/@id"))]
    id: String,

    #[xee(xpath("//person/name/first/text()"))]
    first_name: String,

    #[xee(xpath("//person/name/last/text()"))]
    last_name: String,

    #[xee(xpath("//person/email/text()"))]
    email: Option<String>,

    #[xee(xpath("//person/address/street/text()"))]
    street: Option<String>,

    #[xee(xpath("//person/address/city/text()"))]
    city: Option<String>,
}

#[test]
fn test_person_with_optional_fields() {
    let xml = r#"
        <person id="P001">
            <name>
                <first>John</first>
                <last>Doe</last>
            </name>
            <email>john.doe@example.com</email>
            <address>
                <street>123 Main St</street>
                <city>Anytown</city>
            </address>
        </person>
    "#;

    let extractor = Extractor::default();
    let person: Person = extractor.extract_from_str(xml).unwrap();

    assert_eq!(person.id, "P001");
    assert_eq!(person.first_name, "John");
    assert_eq!(person.last_name, "Doe");
    assert_eq!(person.email, Some("john.doe@example.com".to_string()));
    assert_eq!(person.street, Some("123 Main St".to_string()));
    assert_eq!(person.city, Some("Anytown".to_string()));
}
