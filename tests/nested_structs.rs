use xee_extract::{Extract, Extractor};

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

#[derive(Extract, Debug, PartialEq)]
struct Library {
    #[xee(xpath("//library/@name"))]
    name: String,

    #[xee(extract("//library/books/book"))]
    books: Vec<Book>,
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

#[derive(Extract, Debug, PartialEq)]
struct Company {
    #[xee(xpath("//company/@id"))]
    id: String,

    #[xee(xpath("//company/name/text()"))]
    name: String,

    #[xee(xpath("//company/employees/person/name/first/text()"))]
    employee_first_names: Vec<String>,
}

#[derive(Extract, Debug, PartialEq)]
struct SimpleStruct {
    #[xee(xpath("//id/text()"))]
    id: String,

    #[xee(xpath("//title/text()"))]
    title: String,

    #[xee(extract("//nested"))]
    nested: NestedStruct,
}

#[derive(Extract, Debug, PartialEq)]
struct NestedStruct {
    #[xee(xpath("value/text()"))]
    value: String,

    #[xee(xpath("optional/text()"))]
    optional: Option<String>,
}

#[test]
fn test_book_extraction() {
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
    let book: Book = extractor.extract_from_str(xml).unwrap();

    assert_eq!(book.id, "B001");
    assert_eq!(book.title, "The Rust Programming Language");
    assert_eq!(book.author, "Steve Klabnik");
    assert_eq!(book.price, 39.99);
    assert_eq!(book.genre, Some("fiction".to_string()));
    assert_eq!(book.tags, vec!["programming", "rust", "systems"]);
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

    let extractor = Extractor::new();
    let book: Book = extractor.extract_from_str(xml).unwrap();

    assert_eq!(book.id, "B002");
    assert_eq!(book.title, "Programming Rust");
    assert_eq!(book.author, "Jim Blandy");
    assert_eq!(book.price, 45.50);
    assert_eq!(book.genre, None);
    assert_eq!(book.tags, Vec::<String>::new());
}

#[test]
fn test_person_extraction() {
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

    let extractor = Extractor::new();
    let person: Person = extractor.extract_from_str(xml).unwrap();

    assert_eq!(person.id, "P001");
    assert_eq!(person.first_name, "John");
    assert_eq!(person.last_name, "Doe");
    assert_eq!(person.email, Some("john.doe@example.com".to_string()));
    assert_eq!(person.street, Some("123 Main St".to_string()));
    assert_eq!(person.city, Some("Anytown".to_string()));
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

    let extractor = Extractor::new();
    let person: Person = extractor.extract_from_str(xml).unwrap();

    assert_eq!(person.id, "P002");
    assert_eq!(person.first_name, "Jane");
    assert_eq!(person.last_name, "Smith");
    assert_eq!(person.email, None);
    assert_eq!(person.street, None);
    assert_eq!(person.city, None);
}

#[test]
fn test_complex_nested_extraction() {
    let xml = r#"
        <company id="C001">
            <name>Tech Corp</name>
            <employees>
                <person id="P001">
                    <name>
                        <first>Alice</first>
                        <last>Johnson</last>
                    </name>
                    <email>alice@techcorp.com</email>
                    <address>
                        <street>456 Tech Ave</street>
                        <city>Tech City</city>
                    </address>
                </person>
                <person id="P002">
                    <name>
                        <first>Bob</first>
                        <last>Wilson</last>
                    </name>
                    <address>
                        <street>789 Dev Blvd</street>
                        <city>Dev Town</city>
                    </address>
                </person>
            </employees>
        </company>
    "#;

    let extractor = Extractor::new();
    let company: Company = extractor.extract_from_str(xml).unwrap();

    assert_eq!(company.id, "C001");
    assert_eq!(company.name, "Tech Corp");
    assert_eq!(company.employee_first_names, vec!["Alice", "Bob"]);
}

#[test]
fn test_standalone_book_extraction() {
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
    let book: Book = extractor.extract_from_str(xml).unwrap();

    assert_eq!(book.id, "B001");
    assert_eq!(book.title, "The Rust Programming Language");
    assert_eq!(book.author, "Steve Klabnik");
    assert_eq!(book.price, 39.99);
    assert_eq!(book.genre, Some("fiction".to_string()));
    assert_eq!(book.tags, vec!["programming", "rust", "systems"]);
}

#[test]
fn test_child_book_in_library_context() {
    let xml = r#"
        <library name="My Library">
            <books>
                <book id="B001">
                    <title>Book 1</title>
                    <author>Author 1</author>
                    <price>10.00</price>
                </book>
                <book id="B002">
                    <title>Book 2</title>
                    <author>Author 2</author>
                    <price>20.00</price>
                </book>
            </books>
        </library>
    "#;

    let extractor = Extractor::new();
    let library: Library = extractor.extract_from_str(xml).unwrap();

    assert_eq!(library.name, "My Library");
    assert_eq!(library.books.len(), 2);
    assert_eq!(library.books[0].id, "B001");
    assert_eq!(library.books[0].title, "Book 1");
    assert_eq!(library.books[1].id, "B002");
    assert_eq!(library.books[1].title, "Book 2");
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
    let result: SimpleStruct = extractor.extract_from_str(xml).unwrap();

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
    let result: SimpleStruct = extractor.extract_from_str(xml).unwrap();

    assert_eq!(result.id, "123");
    assert_eq!(result.title, "Test Title");
    assert_eq!(result.nested.value, "Nested Value");
    assert_eq!(result.nested.optional, None);
}
