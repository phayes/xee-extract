use xee_extract::{Extractor, Extract};

#[derive(Extract, Debug, PartialEq)]
#[context("if (self::book) then . else /book")]
struct Book {
    #[xpath("@id")]
    id: String,

    #[xpath("title/text()")]
    title: String,

    #[xpath("author/text()")]
    author: String,

    #[xpath("price/text()")]
    price: f64,

    #[xpath("@genre")]
    genre: Option<String>,

    #[xpath("tags/tag/text()")]
    tags: Vec<String>,
}

#[derive(Extract, Debug, PartialEq)]
struct Library {
    #[xpath("//library/@name")]
    name: String,

    #[extract("//library/books/book")]
    books: Vec<Book>,
}

#[derive(Extract, Debug, PartialEq)]
struct Person {
    #[xpath("//person/@id")]
    id: String,

    #[xpath("//person/name/first/text()")]
    first_name: String,

    #[xpath("//person/name/last/text()")]
    last_name: String,

    #[xpath("//person/email/text()")]
    email: Option<String>,

    #[xpath("//person/address/street/text()")]
    street: Option<String>,

    #[xpath("//person/address/city/text()")]
    city: Option<String>,
}

#[derive(Extract, Debug, PartialEq)]
struct Company {
    #[xpath("//company/@id")]
    id: String,

    #[xpath("//company/name/text()")]
    name: String,

    #[xpath("//company/employees/person/name/first/text()")]
    employee_first_names: Vec<String>,
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
    let book: Book = extractor.extract_one(xml).unwrap();

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
    let book: Book = extractor.extract_one(xml).unwrap();

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
    let person: Person = extractor.extract_one(xml).unwrap();

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
    let person: Person = extractor.extract_one(xml).unwrap();

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
    let company: Company = extractor.extract_one(xml).unwrap();

    assert_eq!(company.id, "C001");
    assert_eq!(company.name, "Tech Corp");
    assert_eq!(company.employee_first_names, vec!["Alice", "Bob"]);
}

#[test]
fn test_xpath_with_variables() {
    let xml = r#"
        <config>
            <api>
                <base_url>https://api.example.com</base_url>
                <version>v1</version>
            </api>
            <user>
                <id>U123</id>
                <name>Test User</name>
            </user>
        </config>
    "#;

    let extractor = Extractor::new()
        .with_variable("env", "production")
        .with_variable("api_version", "v2");

    // Test that variables can be set and accessed
    // Note: The current implementation doesn't use variables in XPath expressions yet
    // This test verifies the variable setting functionality
    assert_eq!(
        extractor.variables.get("env"),
        Some(&"production".to_string())
    );
    assert_eq!(
        extractor.variables.get("api_version"),
        Some(&"v2".to_string())
    );
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
    let book: Book = extractor.extract_one(xml).unwrap();

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
    let library: Library = extractor.extract_one(xml).unwrap();

    assert_eq!(library.name, "My Library");
    assert_eq!(library.books.len(), 2);
    assert_eq!(library.books[0].id, "B001");
    assert_eq!(library.books[0].title, "Book 1");
    assert_eq!(library.books[1].id, "B002");
    assert_eq!(library.books[1].title, "Book 2");
}
