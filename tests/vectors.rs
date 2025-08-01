use xee_extract::{Extractor, Extract};

#[derive(Extract, Debug, PartialEq)]
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

#[derive(Extract, Debug, PartialEq)]
struct Book {
    #[xpath("/book/title/text()")]
    title: String,

    #[xpath("/book/author/text()")]
    author: String,

    #[xpath("/book/year/text()")]
    year: Option<i32>,

    #[xpath("/book/genre/text()")]
    genres: Vec<String>,
}

#[derive(Extract, Debug, PartialEq)]
struct Library {
    #[xpath("//library/@name")]
    name: String,

    #[extract("//library/books/book")]
    books: Vec<Book>,
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
fn test_vector_with_multiple_items() {
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
fn test_book_with_multiple_genres() {
    let xml = r#"
        <book>
            <title>The Rust Programming Language</title>
            <author>Steve Klabnik</author>
            <year>2018</year>
            <genre>Programming</genre>
            <genre>Reference</genre>
        </book>
    "#;

    let extractor = Extractor::new();
    let book: Book = extractor.extract_one(xml).unwrap();

    assert_eq!(book.title, "The Rust Programming Language");
    assert_eq!(book.author, "Steve Klabnik");
    assert_eq!(book.year, Some(2018));
    assert_eq!(book.genres, vec!["Programming", "Reference"]);
}

#[test]
fn test_book_with_single_genre() {
    let xml = r#"
        <book>
            <title>Programming Rust</title>
            <author>Jim Blandy</author>
            <year>2021</year>
            <genre>Programming</genre>
        </book>
    "#;

    let extractor = Extractor::new();
    let book: Book = extractor.extract_one(xml).unwrap();

    assert_eq!(book.title, "Programming Rust");
    assert_eq!(book.author, "Jim Blandy");
    assert_eq!(book.year, Some(2021));
    assert_eq!(book.genres, vec!["Programming"]);
}

#[test]
fn test_empty_library() {
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

#[test]
fn test_company_with_multiple_employees() {
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