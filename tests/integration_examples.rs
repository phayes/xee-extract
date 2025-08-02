use xee_extract::{Extractor, Extract};
use std::str::FromStr;

// Test 1: Basic Extraction
#[derive(Extract, Debug, PartialEq)]
struct TestPerson {
    #[xee(xpath("//name/text()"))]
    name: String,

    #[xee(xpath("//age/text()"))]
    age: u32,

    #[xee(xpath("//email/text()"))]
    email: Option<String>,

    #[xee(xpath("//hobbies/hobby/text()"))]
    hobbies: Vec<String>,
}

#[derive(Extract, Debug, PartialEq)]
struct TestCompany {
    #[xee(xpath("//company/@id"))]
    id: String,

    #[xee(xpath("//company/name/text()"))]
    name: String,

    #[xee(xpath("//company/employees/employee/name/text()"))]
    employee_names: Vec<String>,

    #[xee(xpath("//company/address/city/text()"))]
    city: Option<String>,
}

// Test 2: Custom ExtractValue
#[derive(Debug, PartialEq)]
enum TestUserStatus {
    Active,
    Inactive,
    Pending,
    Suspended,
}

impl FromStr for TestUserStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "active" => Ok(TestUserStatus::Active),
            "inactive" => Ok(TestUserStatus::Inactive),
            "pending" => Ok(TestUserStatus::Pending),
            "suspended" => Ok(TestUserStatus::Suspended),
            _ => Err(format!("Unknown status: {}", s)),
        }
    }
}

#[derive(Extract, Debug, PartialEq)]
struct TestUserProfile {
    #[xee(xpath("//name/text()"))]
    name: String,

    #[xee(xpath("//status/text()"))]
    status: TestUserStatus,

    #[xee(xpath("//location/text()"))]
    location: Option<String>,
}

// Test 3: Namespaces
#[derive(Extract, Debug, PartialEq)]
#[xee(ns(atom = "http://www.w3.org/2005/Atom"))]
struct TestAtomFeed {
    #[xee(xpath("//atom:feed/atom:title/text()"))]
    title: String,

    #[xee(xpath("//atom:feed/atom:entry/atom:title/text()"))]
    entry_titles: Vec<String>,
}

// Test 4: Contexts
#[derive(Extract, Debug, PartialEq)]
#[xee(context("(//book)[1]"))]
struct TestBook {
    #[xee(xpath("@id"))]
    id: String,

    #[xee(xpath("title/text()"))]
    title: String,

    #[xee(xpath("author/text()"))]
    author: String,

    #[xee(xpath("price/text()"))]
    price: f64,
}

// Test 5: Nested Structs
#[derive(Extract, Debug, PartialEq)]
struct TestAuthor {
    #[xee(xpath("name/text()"))]
    name: String,

    #[xee(xpath("email/text()"))]
    email: Option<String>,
}

#[derive(Extract, Debug, PartialEq)]
struct TestBookWithAuthor {
    #[xee(xpath("//book/@id"))]
    id: String,

    #[xee(xpath("//book/title/text()"))]
    title: String,

    #[xee(extract("//book/author"))]
    author: TestAuthor,
}

// Test 6: Raw XML
#[derive(Extract, Debug, PartialEq)]
struct TestRawXMLData {
    #[xee(xpath("//title/text()"))]
    title: String,

    #[xee(xml("//content"))]
    content_xml: String,

    #[xee(xml("//metadata"))]
    metadata_xml: Option<String>,
}

// Test 7: Binary Handling
#[derive(Extract, Debug, PartialEq)]
#[xee(ns(xs = "http://www.w3.org/2001/XMLSchema"))]
struct TestBinaryData {
    #[xee(xpath("string(//base64-data/text()) cast as xs:base64Binary"))]
    base64_data: Vec<u8>,

    #[xee(xpath("string(//hex-data/text()) cast as xs:hexBinary"))]
    hex_data: Vec<u8>,
}

#[test]
fn test_basic_extraction() {
    let xml = r#"
        <person>
            <name>John Doe</name>
            <age>30</age>
            <email>john@example.com</email>
            <hobbies>
                <hobby>reading</hobby>
                <hobby>swimming</hobby>
                <hobby>coding</hobby>
            </hobbies>
        </person>
    "#;

    let extractor = Extractor::new();
    let person: TestPerson = extractor.extract_one(xml).unwrap();

    assert_eq!(person.name, "John Doe");
    assert_eq!(person.age, 30);
    assert_eq!(person.email, Some("john@example.com".to_string()));
    assert_eq!(person.hobbies, vec!["reading", "swimming", "coding"]);
}

#[test]
fn test_company_extraction() {
    let xml = r#"
        <company id="C001">
            <name>Tech Corp</name>
            <employees>
                <employee>
                    <name>Alice Johnson</name>
                </employee>
                <employee>
                    <name>Bob Smith</name>
                </employee>
            </employees>
            <address>
                <city>San Francisco</city>
            </address>
        </company>
    "#;

    let extractor = Extractor::new();
    let company: TestCompany = extractor.extract_one(xml).unwrap();

    assert_eq!(company.id, "C001");
    assert_eq!(company.name, "Tech Corp");
    assert_eq!(company.employee_names, vec!["Alice Johnson", "Bob Smith"]);
    assert_eq!(company.city, Some("San Francisco".to_string()));
}

#[test]
fn test_custom_extract_value() {
    let xml = r#"
        <profile>
            <name>Alice Johnson</name>
            <status>active</status>
            <location>San Francisco</location>
        </profile>
    "#;

    let extractor = Extractor::new();
    let profile: TestUserProfile = extractor.extract_one(xml).unwrap();

    assert_eq!(profile.name, "Alice Johnson");
    assert_eq!(profile.status, TestUserStatus::Active);
    assert_eq!(profile.location, Some("San Francisco".to_string()));
}

#[test]
fn test_namespaces() {
    let xml = r#"
        <feed xmlns="http://www.w3.org/2005/Atom">
            <title>My Blog</title>
            <entry>
                <title>Getting Started with Rust</title>
            </entry>
            <entry>
                <title>Advanced XPath Techniques</title>
            </entry>
        </feed>
    "#;

    let extractor = Extractor::new();
    let feed: TestAtomFeed = extractor.extract_one(xml).unwrap();

    assert_eq!(feed.title, "My Blog");
    assert_eq!(feed.entry_titles, vec!["Getting Started with Rust", "Advanced XPath Techniques"]);
}

#[test]
fn test_contexts() {
    let xml = r#"
        <library>
            <book id="B001" genre="programming">
                <title>The Rust Programming Language</title>
                <author>Steve Klabnik</author>
                <price>39.99</price>
            </book>
        </library>
    "#;

    let extractor = Extractor::new();
    let book: TestBook = extractor.extract_one(xml).unwrap();

    assert_eq!(book.id, "B001");
    assert_eq!(book.title, "The Rust Programming Language");
    assert_eq!(book.author, "Steve Klabnik");
    assert_eq!(book.price, 39.99);
}

#[test]
fn test_nested_structs() {
    let xml = r#"
        <book id="B001" genre="programming">
            <title>The Rust Programming Language</title>
            <author>
                <name>Steve Klabnik</name>
                <email>steve@rust-lang.org</email>
            </author>
        </book>
    "#;

    let extractor = Extractor::new();
    let book: TestBookWithAuthor = extractor.extract_one(xml).unwrap();

    assert_eq!(book.id, "B001");
    assert_eq!(book.title, "The Rust Programming Language");
    assert_eq!(book.author.name, "Steve Klabnik");
    assert_eq!(book.author.email, Some("steve@rust-lang.org".to_string()));
}

#[test]
fn test_raw_xml() {
    let xml = r#"
        <article>
            <title>Sample Article</title>
            <content>
                <p>This is a <strong>paragraph</strong> with <em>formatting</em>.</p>
                <ul>
                    <li>Item 1</li>
                    <li>Item 2</li>
                </ul>
            </content>
            <metadata>
                <author>John Doe</author>
                <date>2024-01-15</date>
            </metadata>
        </article>
    "#;

    let extractor = Extractor::new();
    let data: TestRawXMLData = extractor.extract_one(xml).unwrap();

    assert_eq!(data.title, "Sample Article");
    assert!(data.content_xml.contains("<p>This is a <strong>paragraph</strong>"));
    assert!(data.content_xml.contains("<ul>"));
    assert!(data.metadata_xml.is_some());
    assert!(data.metadata_xml.unwrap().contains("<author>John Doe</author>"));
}

#[test]
fn test_binary_handling() {
    let xml = r#"
        <root>
            <base64-data>SGVsbG8gV29ybGQ=</base64-data>
            <hex-data>48656C6C6F20576F726C64</hex-data>
        </root>
    "#;

    let extractor = Extractor::new();
    let data: TestBinaryData = extractor.extract_one(xml).unwrap();

    // "Hello World" in base64 and hex
    let expected = b"Hello World".to_vec();
    assert_eq!(data.base64_data, expected);
    assert_eq!(data.hex_data, expected);
}

#[test]
fn test_optional_fields() {
    let xml = r#"
        <person>
            <name>Jane Smith</name>
            <age>25</age>
            <!-- Missing email -->
            <hobbies>
                <hobby>painting</hobby>
            </hobbies>
        </person>
    "#;

    let extractor = Extractor::new();
    let person: TestPerson = extractor.extract_one(xml).unwrap();

    assert_eq!(person.name, "Jane Smith");
    assert_eq!(person.age, 25);
    assert_eq!(person.email, None);
    assert_eq!(person.hobbies, vec!["painting"]);
}

#[test]
fn test_error_handling_invalid_xpath() {
    let xml = r#"
        <person>
            <name>John Doe</name>
            <!-- Missing age field -->
        </person>
    "#;

    let extractor = Extractor::new();
    let result = extractor.extract_one::<TestPerson>(xml);
    
    assert!(result.is_err());
}

#[test]
fn test_error_handling_invalid_custom_type() {
    let xml = r#"
        <profile>
            <name>Invalid User</name>
            <status>invalid_status</status>
        </profile>
    "#;

    let extractor = Extractor::new();
    let result = extractor.extract_one::<TestUserProfile>(xml);
    
    assert!(result.is_err());
}

#[test]
fn test_error_handling_invalid_binary() {
    let xml = r#"
        <root>
            <base64-data>Invalid base64 data!</base64-data>
        </root>
    "#;

    let extractor = Extractor::new();
    let result = extractor.extract_one::<TestBinaryData>(xml);
    
    assert!(result.is_err());
}

#[test]
fn test_empty_vectors() {
    let xml = r#"
        <person>
            <name>Empty Person</name>
            <age>30</age>
            <email>empty@example.com</email>
            <hobbies>
                <!-- No hobbies -->
            </hobbies>
        </person>
    "#;

    let extractor = Extractor::new();
    let person: TestPerson = extractor.extract_one(xml).unwrap();

    assert_eq!(person.name, "Empty Person");
    assert_eq!(person.age, 30);
    assert_eq!(person.email, Some("empty@example.com".to_string()));
    assert_eq!(person.hobbies, Vec::<String>::new());
}

#[test]
fn test_multiple_entries() {
    let xml = r#"
        <library>
            <book id="B001">
                <title>Book 1</title>
                <author>Author 1</author>
                <price>29.99</price>
            </book>
            <book id="B002">
                <title>Book 2</title>
                <author>Author 2</author>
                <price>39.99</price>
            </book>
        </library>
    "#;

    let extractor = Extractor::new();
    let book1: TestBook = extractor.extract_one(xml).unwrap();
    
    // Should extract the first book due to context
    assert_eq!(book1.id, "B001");
    assert_eq!(book1.title, "Book 1");
    assert_eq!(book1.author, "Author 1");
    assert_eq!(book1.price, 29.99);
}

#[test]
fn test_namespace_error_handling() {
    let xml = r#"
        <feed>
            <title>No Namespace Feed</title>
            <entry>
                <title>This won't work without namespace</title>
            </entry>
        </feed>
    "#;

    let extractor = Extractor::new();
    let result = extractor.extract_one::<TestAtomFeed>(xml);
    
    assert!(result.is_err());
}

#[test]
fn test_missing_nested_elements() {
    let xml = r#"
        <book id="B002" genre="fiction">
            <title>Incomplete Book</title>
            <!-- Missing author element -->
        </book>
    "#;

    let extractor = Extractor::new();
    let result = extractor.extract_one::<TestBookWithAuthor>(xml);
    
    assert!(result.is_err());
} 