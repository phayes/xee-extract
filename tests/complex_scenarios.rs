use xee_extract::{Extractor, Extract};

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

    #[xee(extract("metadata"))]
    metadata: BookMetadata,
}

#[derive(Extract, Debug, PartialEq)]
struct BookMetadata {
    #[xee(xpath("isbn/text()"))]
    isbn: String,

    #[xee(xpath("publisher/text()"))]
    publisher: Option<String>,

    #[xee(xpath("publication_date/text()"))]
    publication_date: Option<String>,

    #[xee(xpath("reviews/text()"))]
    reviews: Option<String>,
}

#[derive(Extract, Debug, PartialEq)]
struct LibraryWithComplexBooks {
    #[xee(xpath("//library/@name"))]
    name: String,

    #[xee(xpath("//library/@location"))]
    location: Option<String>,

    #[xee(extract("//library/books/book"))]
    books: Vec<Book>,
}

#[derive(Extract, Debug, PartialEq)]
struct CompanyWithDepartments {
    #[xee(xpath("//company/@id"))]
    id: String,

    #[xee(xpath("//company/name/text()"))]
    name: String,

    #[xee(extract("//company/departments/department"))]
    departments: Vec<Department>,
}

#[derive(Extract, Debug, PartialEq)]
struct Department {
    #[xee(xpath("@id"))]
    id: String,

    #[xee(xpath("name/text()"))]
    name: String,

    #[xee(xpath("manager/name/text()"))]
    manager_name: String,

    #[xee(xpath("manager/email/text()"))]
    manager_email: Option<String>,

    #[xee(xpath("employees/employee/name/text()"))]
    employee_names: Vec<String>,
}

#[derive(Extract, Debug, PartialEq)]
struct ConfigStruct {
    #[xee(xpath("//config/api/base_url/text()"))]
    base_url: String,

    #[xee(xpath("//config/api/version/text()"))]
    version: String,

    #[xee(xpath("//config/user/id/text()"))]
    user_id: String,

    #[xee(xpath("//config/user/name/text()"))]
    user_name: String,
}

#[test]
fn test_complex_book_with_metadata() {
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
            <metadata>
                <isbn>978-1492052590</isbn>
                <publisher>No Starch Press</publisher>
                <publication_date>2018-08-01</publication_date>
                <reviews>Excellent book for learning Rust</reviews>
            </metadata>
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

    assert_eq!(book.metadata.isbn, "978-1492052590");
    assert_eq!(book.metadata.publisher, Some("No Starch Press".to_string()));
    assert_eq!(book.metadata.publication_date, Some("2018-08-01".to_string()));
    assert_eq!(book.metadata.reviews, Some("Excellent book for learning Rust".to_string()));
}

#[test]
fn test_library_with_complex_books() {
    let xml = r#"
        <library name="Tech Library" location="San Francisco">
            <books>
                <book id="B001" genre="programming">
                    <title>The Rust Programming Language</title>
                    <author>Steve Klabnik</author>
                    <price>39.99</price>
                    <tags>
                        <tag>rust</tag>
                        <tag>programming</tag>
                    </tags>
                    <metadata>
                        <isbn>978-1492052590</isbn>
                        <publisher>No Starch Press</publisher>
                    </metadata>
                </book>
                <book id="B002" genre="programming">
                    <title>Programming Rust</title>
                    <author>Jim Blandy</author>
                    <price>45.50</price>
                    <tags>
                        <tag>rust</tag>
                        <tag>programming</tag>
                    </tags>
                    <metadata>
                        <isbn>978-1492052591</isbn>
                        <publisher>O'Reilly</publisher>
                    </metadata>
                </book>
            </books>
        </library>
    "#;

    let extractor = Extractor::new();
    let library: LibraryWithComplexBooks = extractor.extract_from_str(xml).unwrap();

    assert_eq!(library.name, "Tech Library");
    assert_eq!(library.location, Some("San Francisco".to_string()));
    assert_eq!(library.books.len(), 2);

    let first_book = &library.books[0];
    assert_eq!(first_book.id, "B001");
    assert_eq!(first_book.title, "The Rust Programming Language");
    assert_eq!(first_book.metadata.isbn, "978-1492052590");
    assert_eq!(first_book.metadata.publisher, Some("No Starch Press".to_string()));

    let second_book = &library.books[1];
    assert_eq!(second_book.id, "B002");
    assert_eq!(second_book.title, "Programming Rust");
    assert_eq!(second_book.metadata.isbn, "978-1492052591");
    assert_eq!(second_book.metadata.publisher, Some("O'Reilly".to_string()));
}

#[test]
fn test_company_with_departments() {
    let xml = r#"
        <company id="C001">
            <name>Tech Corp</name>
            <departments>
                <department id="D001">
                    <name>Engineering</name>
                    <manager>
                        <name>Alice Johnson</name>
                        <email>alice@techcorp.com</email>
                    </manager>
                    <employees>
                        <employee>
                            <name>Bob Wilson</name>
                        </employee>
                        <employee>
                            <name>Carol Davis</name>
                        </employee>
                    </employees>
                </department>
                <department id="D002">
                    <name>Marketing</name>
                    <manager>
                        <name>David Brown</name>
                    </manager>
                    <employees>
                        <employee>
                            <name>Eve Smith</name>
                        </employee>
                    </employees>
                </department>
            </departments>
        </company>
    "#;

    let extractor = Extractor::new();
    let company: CompanyWithDepartments = extractor.extract_from_str(xml).unwrap();

    assert_eq!(company.id, "C001");
    assert_eq!(company.name, "Tech Corp");
    assert_eq!(company.departments.len(), 2);

    let engineering = &company.departments[0];
    assert_eq!(engineering.id, "D001");
    assert_eq!(engineering.name, "Engineering");
    assert_eq!(engineering.manager_name, "Alice Johnson");
    assert_eq!(engineering.manager_email, Some("alice@techcorp.com".to_string()));
    assert_eq!(engineering.employee_names, vec!["Bob Wilson", "Carol Davis"]);

    let marketing = &company.departments[1];
    assert_eq!(marketing.id, "D002");
    assert_eq!(marketing.name, "Marketing");
    assert_eq!(marketing.manager_name, "David Brown");
    assert_eq!(marketing.manager_email, None);
    assert_eq!(marketing.employee_names, vec!["Eve Smith"]);
}

#[test]
fn test_complex_scenario_without_variables() {
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

    let extractor = Extractor::new();
    let result: ConfigStruct = extractor.extract_from_str(xml).unwrap();
    
    assert_eq!(result.base_url, "https://api.example.com");
    assert_eq!(result.version, "v1");
    assert_eq!(result.user_id, "U123");
    assert_eq!(result.user_name, "Test User");
} 