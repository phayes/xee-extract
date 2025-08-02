//! Example 6: Nested Structs
//! 
//! This example demonstrates how to use nested structs with the extract attribute
//! to handle complex XML structures with multiple levels of data.

use xee_extract::{Extractor, Extract};

/// Nested struct for book metadata
#[derive(Extract)]
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

/// Nested struct for author information
#[derive(Extract)]
struct Author {
    #[xee(xpath("name/text()"))]
    name: String,

    #[xee(xpath("email/text()"))]
    email: Option<String>,

    #[xee(xpath("bio/text()"))]
    bio: Option<String>,
}

/// Main book struct with nested components
#[derive(Extract)]
#[xee(context("/book"))]
struct Book {
    #[xee(xpath("@id"))]
    id: String,

    #[xee(xpath("title/text()"))]
    title: String,

    #[xee(xpath("price/text()"))]
    price: f64,

    #[xee(xpath("@genre"))]
    genre: Option<String>,

    #[xee(extract("author"))]
    author: Author,

    #[xee(extract("metadata"))]
    metadata: BookMetadata,
}

/// Nested struct for department information
#[derive(Extract)]
struct Department {
    #[xee(xpath("@id"))]
    id: String,

    #[xee(xpath("name/text()"))]
    name: String,

    #[xee(extract("manager"))]
    manager: Author,

    #[xee(xpath("employees/employee/name/text()"))]
    employee_names: Vec<String>,
}

/// Company struct with nested departments
#[derive(Extract)]
#[xee(context("/company"))]
struct Company {
    #[xee(xpath("@id"))]
    id: String,

    #[xee(xpath("name/text()"))]
    name: String,

    #[xee(xpath("location/text()"))]
    location: Option<String>,

    #[xee(extract("departments/department"))]
    departments: Vec<Department>,
}

/// Nested struct for order items
#[derive(Extract)]
struct OrderItem {
    #[xee(xpath("@sku"))]
    sku: String,

    #[xee(xpath("name/text()"))]
    name: String,

    #[xee(xpath("quantity/text()"))]
    quantity: u32,

    #[xee(xpath("price/text()"))]
    price: f64,
}

/// Order struct with nested items and customer
#[derive(Extract)]
#[xee(context("/order"))]
struct Order {
    #[xee(xpath("@order_id"))]
    order_id: String,

    #[xee(xpath("order_date/text()"))]
    order_date: String,

    #[xee(extract("customer"))]
    customer: Author,

    #[xee(extract("items/item"))]
    items: Vec<OrderItem>,

    #[xee(xpath("total/text()"))]
    total: f64,
}

fn main() {
    // Example 1: Book with nested author and metadata
    let book_xml = r#"
        <book id="B001" genre="programming">
            <title>The Rust Programming Language</title>
            <price>39.99</price>
            <author>
                <name>Steve Klabnik</name>
                <email>steve@rust-lang.org</email>
                <bio>Rust community team member</bio>
            </author>
            <metadata>
                <isbn>978-1492052590</isbn>
                <publisher>No Starch Press</publisher>
                <publication_date>2018-08-01</publication_date>
                <reviews>Excellent book for learning Rust</reviews>
            </metadata>
        </book>
    "#;

    let extractor = Extractor::new();
    let book: Book = extractor.extract_one(book_xml).unwrap();

    println!("Book with nested structs:");
    println!("  ID: {}", book.id);
    println!("  Title: {}", book.title);
    println!("  Price: {}", book.price);
    println!("  Genre: {:?}", book.genre);
    println!("  Author: {} ({})", book.author.name, book.author.email.as_deref().unwrap_or("No email"));
    println!("  Author Bio: {:?}", book.author.bio);
    println!("  ISBN: {}", book.metadata.isbn);
    println!("  Publisher: {:?}", book.metadata.publisher);
    println!("  Publication Date: {:?}", book.metadata.publication_date);
    println!("  Reviews: {:?}", book.metadata.reviews);
    println!();

    // Example 2: Company with nested departments
    let company_xml = r#"
        <company id="C001">
            <name>Tech Corp</name>
            <location>San Francisco</location>
            <departments>
                <department id="D001">
                    <name>Engineering</name>
                    <manager>
                        <name>Alice Johnson</name>
                        <email>alice@techcorp.com</email>
                        <bio>Senior Engineering Manager</bio>
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
                        <email>david@techcorp.com</email>
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

    let company: Company = extractor.extract_one(company_xml).unwrap();

    println!("Company with nested departments:");
    println!("  ID: {}", company.id);
    println!("  Name: {}", company.name);
    println!("  Location: {:?}", company.location);
    println!("  Departments:");
    for dept in &company.departments {
        println!("    - {} (ID: {})", dept.name, dept.id);
        println!("      Manager: {} ({})", dept.manager.name, dept.manager.email.as_deref().unwrap_or("No email"));
        println!("      Employees: {:?}", dept.employee_names);
    }
    println!();

    // Example 3: Order with nested items and customer
    let order_xml = r#"
        <order order_id="ORD001">
            <order_date>2024-01-15</order_date>
            <customer>
                <name>John Doe</name>
                <email>john@example.com</email>
                <bio>Premium customer</bio>
            </customer>
            <items>
                <item sku="SKU001">
                    <name>Laptop</name>
                    <quantity>1</quantity>
                    <price>999.99</price>
                </item>
                <item sku="SKU002">
                    <name>Mouse</name>
                    <quantity>2</quantity>
                    <price>29.99</price>
                </item>
                <item sku="SKU003">
                    <name>Keyboard</name>
                    <quantity>1</quantity>
                    <price>89.99</price>
                </item>
            </items>
            <total>1149.96</total>
        </order>
    "#;

    let order: Order = extractor.extract_one(order_xml).unwrap();

    println!("Order with nested items and customer:");
    println!("  Order ID: {}", order.order_id);
    println!("  Order Date: {}", order.order_date);
    println!("  Customer: {} ({})", order.customer.name, order.customer.email.as_deref().unwrap_or("No email"));
    println!("  Items:");
    for item in &order.items {
        println!("    - {} (SKU: {}) x{} @ ${:.2}", item.name, item.sku, item.quantity, item.price);
    }
    println!("  Total: ${:.2}", order.total);
    println!();

} 