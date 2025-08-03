//! Example 9: Multi-Document Extraction
//!
//! This example demonstrates how to extract data that spans multiple XML documents
//! using the doc() function to cross-reference information between documents.

use xee_extract::{Extract, Extractor};
use xee_xpath::Documents;

/// A struct that combines user profile data with permissions from a separate document
#[derive(Extract)]
struct UserWithPermissions {
    #[xee(xpath("//user/@id"))]
    user_id: String,

    #[xee(xpath("//user/name/text()"))]
    name: String,

    #[xee(xpath("//user/email/text()"))]
    email: String,

    // Cross-document extraction using doc() function
    #[xee(xpath("let $uid := //user/@id return doc('http://example.com/permissions.xml')//permissions[@user_id = $uid]/access_level/text()"))]
    access_level: String,

    #[xee(xpath("let $uid := //user/@id return doc('http://example.com/permissions.xml')//permissions[@user_id = $uid]/roles/role/text()"))]
    roles: Vec<String>,
}

/// A struct that combines product catalog data with inventory information
#[derive(Extract)]
struct ProductWithInventory {
    #[xee(xpath("//product[1]/@id"))]
    product_id: String,

    #[xee(xpath("//product[1]/name/text()"))]
    name: String,

    #[xee(xpath("//product[1]/description/text()"))]
    description: String,

    // Cross-document inventory data
    #[xee(xpath("let $pid := //product[1]/@id return doc('http://example.com/inventory.xml')//inventory[@product_id = $pid]/stock_level/text()"))]
    stock_level: i32,

    #[xee(xpath("let $pid := //product[1]/@id return doc('http://example.com/inventory.xml')//inventory[@product_id = $pid]/price/text()"))]
    price: f64,
}

/// A struct demonstrating simple cross-document extraction without variables
#[derive(Extract)]
struct SimpleCrossDocument {
    #[xee(xpath("//user/@id"))]
    user_id: String,

    #[xee(xpath("//user/name/text()"))]
    name: String,

    // Simple cross-document extraction without variables
    #[xee(xpath("doc('http://example.com/extra.xml')/extra/value/text()"))]
    extra_value: String,
}

fn main() {
    // Create a documents collection to hold multiple XML documents
    let mut documents = Documents::new();

    // Add user profile document
    let user_doc = documents
        .add_string(
            "http://example.com/users.xml".try_into().unwrap(),
            r#"<users>
                <user id="user123">
                    <name>John Doe</name>
                    <email>john@example.com</email>
                </user>
            </users>"#,
        )
        .unwrap();

    // Add permissions document (separate file)
    documents
        .add_string(
            "http://example.com/permissions.xml".try_into().unwrap(),
            r#"<permissions>
                <permissions user_id="user123">
                    <access_level>admin</access_level>
                    <roles>
                        <role>developer</role>
                        <role>admin</role>
                        <role>reviewer</role>
                    </roles>
                </permissions>
            </permissions>"#,
        )
        .unwrap();

    // Extract user data with cross-document permissions
    let extractor = Extractor::new();
    let user: UserWithPermissions = extractor
        .extract_from_docs(&mut documents, &user_doc)
        .unwrap();

    println!("User with permissions:");
    println!("  ID: {}", user.user_id);
    println!("  Name: {}", user.name);
    println!("  Email: {}", user.email);
    println!("  Access Level: {}", user.access_level);
    println!("  Roles: {:?}", user.roles);
    println!();

    // Example 2: Product catalog with inventory
    let catalog_doc = documents
        .add_string(
            "http://example.com/catalog.xml".try_into().unwrap(),
            r#"<catalog>
                <product id="prod001">
                    <name>Laptop Computer</name>
                    <description>High-performance laptop with latest specs</description>
                </product>
            </catalog>"#,
        )
        .unwrap();

    // Add inventory document
    documents
        .add_string(
            "http://example.com/inventory.xml".try_into().unwrap(),
            r#"<inventory>
                <inventory product_id="prod001">
                    <stock_level>15</stock_level>
                    <price>1299.99</price>
                </inventory>
            </inventory>"#,
        )
        .unwrap();

    // Extract product with inventory data
    let product: ProductWithInventory = extractor
        .extract_from_docs(&mut documents, &catalog_doc)
        .unwrap();

    println!("Product with inventory:");
    println!("  ID: {}", product.product_id);
    println!("  Name: {}", product.name);
    println!("  Description: {}", product.description);
    println!("  Stock Level: {}", product.stock_level);
    println!("  Price: ${:.2}", product.price);
    println!();

    // Example 3: Simple cross-document extraction without variables
    let simple_doc = documents
        .add_string(
            "http://example.com/simple.xml".try_into().unwrap(),
            r#"<users>
                <user id="user456">
                    <name>Jane Smith</name>
                </user>
            </users>"#,
        )
        .unwrap();

    // Add extra document with simple value
    documents
        .add_string(
            "http://example.com/extra.xml".try_into().unwrap(),
            r#"<extra>
                <value>additional information</value>
            </extra>"#,
        )
        .unwrap();

    // Extract simple cross-document data
    let simple: SimpleCrossDocument = extractor
        .extract_from_docs(&mut documents, &simple_doc)
        .unwrap();

    println!("Simple cross-document extraction:");
    println!("  User ID: {}", simple.user_id);
    println!("  Name: {}", simple.name);
    println!("  Extra Value: {}", simple.extra_value);
}
