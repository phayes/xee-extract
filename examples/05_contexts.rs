//! Example 5: Contexts
//!
//! This example demonstrates how to use context in xee-extract to set the
//! starting point for XPath expressions and handle conditional logic.

use xee_extract::{Extract, Extractor};

/// Struct with simple context - extracts from a specific element
#[derive(Extract)]
#[xee(context("(//book)[1]"))]
struct Book {
    #[xee(xpath("@id"))]
    id: String,

    #[xee(xpath("title"))]
    title: String,

    #[xee(xpath("author"))]
    author: String,

    #[xee(xpath("price"))]
    price: f64,

    #[xee(xpath("@genre"))]
    genre: Option<String>,
}

/// Struct with conditional context - handles different XML structures
#[derive(Extract)]
#[xee(context("if (self::book) then . else //book"))]
struct FlexibleBook {
    #[xee(xpath("@id"))]
    id: String,

    #[xee(xpath("title"))]
    title: String,

    #[xee(xpath("author"))]
    author: String,

    #[xee(xpath("price"))]
    price: f64,
}

/// Struct with multiple context options
#[derive(Extract)]
#[xee(context("(//entry)[1]"))]
struct FirstEntry {
    #[xee(xpath("@id"))]
    id: String,

    #[xee(xpath("title"))]
    title: String,

    #[xee(xpath("author/name"))]
    author_name: String,
}

/// Struct for extracting from a specific position in a list
#[derive(Extract)]
#[xee(context("//products/product[position() = 1]"))]
struct FirstProduct {
    #[xee(xpath("@sku"))]
    sku: String,

    #[xee(xpath("name"))]
    name: String,

    #[xee(xpath("price"))]
    price: f64,

    #[xee(xpath("category"))]
    category: Option<String>,
}

/// Struct for extracting with complex context logic
#[derive(Extract)]
#[xee(context("//users/user[@type = 'admin']"))]
struct AdminUser {
    #[xee(xpath("@id"))]
    id: String,

    #[xee(xpath("name"))]
    name: String,

    #[xee(xpath("email"))]
    email: String,

    #[xee(xpath("permissions/permission"))]
    permissions: Vec<String>,
}

fn main() {
    // Example 1: Simple context - extracting from a book element
    let book_xml = r#"
        <library>
            <book id="B001" genre="fiction">
                <title>The Rust Programming Language</title>
                <author>Steve Klabnik</author>
                <price>39.99</price>
            </book>
            <book id="B002" genre="non-fiction">
                <title>Programming Rust</title>
                <author>Jim Blandy</author>
                <price>45.50</price>
            </book>
        </library>
    "#;

    let extractor = Extractor::new();
    let book: Book = extractor.extract_from_str(book_xml).unwrap();

    println!("Book with simple context:");
    println!("  ID: {}", book.id);
    println!("  Title: {}", book.title);
    println!("  Author: {}", book.author);
    println!("  Price: {}", book.price);
    println!("  Genre: {:?}", book.genre);
    println!();

    // Example 2: Flexible context - works with different XML structures
    let direct_book_xml = r#"
        <book id="B003" genre="programming">
            <title>Effective Rust</title>
            <author>Carol Nichols</author>
            <price>29.99</price>
        </book>
    "#;

    let book: FlexibleBook = extractor.extract_from_str(direct_book_xml).unwrap();

    println!("Flexible book (direct structure):");
    println!("  ID: {}", book.id);
    println!("  Title: {}", book.title);
    println!("  Author: {}", book.author);
    println!("  Price: {}", book.price);
    println!();

    let library_book_xml = r#"
        <library>
            <book id="B004" genre="programming">
                <title>Rust in Action</title>
                <author>Tim McNamara</author>
                <price>49.99</price>
            </book>
        </library>
    "#;

    let book: FlexibleBook = extractor.extract_from_str(library_book_xml).unwrap();

    println!("Flexible book (library structure):");
    println!("  ID: {}", book.id);
    println!("  Title: {}", book.title);
    println!("  Author: {}", book.author);
    println!("  Price: {}", book.price);
    println!();

    // Example 3: First entry context
    let entries_xml = r#"
        <feed>
            <entry id="E001">
                <title>First Entry</title>
                <author>
                    <name>Alice Johnson</name>
                </author>
            </entry>
            <entry id="E002">
                <title>Second Entry</title>
                <author>
                    <name>Bob Smith</name>
                </author>
            </entry>
        </feed>
    "#;

    let entry: FirstEntry = extractor.extract_from_str(entries_xml).unwrap();

    println!("First entry with context:");
    println!("  ID: {}", entry.id);
    println!("  Title: {}", entry.title);
    println!("  Author: {}", entry.author_name);
    println!();

    // Example 4: First product with position context
    let products_xml = r#"
        <catalog>
            <products>
                <product sku="SKU001">
                    <name>Laptop</name>
                    <price>999.99</price>
                    <category>Electronics</category>
                </product>
                <product sku="SKU002">
                    <name>Mouse</name>
                    <price>29.99</price>
                    <category>Electronics</category>
                </product>
            </products>
        </catalog>
    "#;

    let product: FirstProduct = extractor.extract_from_str(products_xml).unwrap();

    println!("First product with position context:");
    println!("  SKU: {}", product.sku);
    println!("  Name: {}", product.name);
    println!("  Price: {}", product.price);
    println!("  Category: {:?}", product.category);
    println!();

    // Example 5: Admin user with conditional context
    let users_xml = r#"
        <users>
            <user id="U001" type="admin">
                <name>Admin User</name>
                <email>admin@example.com</email>
                <permissions>
                    <permission>read</permission>
                    <permission>write</permission>
                    <permission>delete</permission>
                </permissions>
            </user>
            <user id="U002" type="user">
                <name>Regular User</name>
                <email>user@example.com</email>
                <permissions>
                    <permission>read</permission>
                </permissions>
            </user>
        </users>
    "#;

    let admin: AdminUser = extractor.extract_from_str(users_xml).unwrap();

    println!("Admin user with conditional context:");
    println!("  ID: {}", admin.id);
    println!("  Name: {}", admin.name);
    println!("  Email: {}", admin.email);
    println!("  Permissions: {:?}", admin.permissions);
    println!();
}
