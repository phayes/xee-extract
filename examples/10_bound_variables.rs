//! Example 10: Bound Variables
//!
//! This example demonstrates how to use bound variables with bind_value to
//! dynamically extract data from XML using variable values in XPath expressions.

use xee_extract::{Extract, Extractor};

/// A struct demonstrating basic variable binding
#[derive(Extract)]
struct UserProfile {
    #[xee(xpath("$user_name"))]
    user_name: String,

    #[xee(xpath("$user_age"))]
    user_age: i32,

    #[xee(xpath("$is_active"))]
    is_active: bool,

    #[xee(xpath("$score"))]
    score: f64,
}

/// A struct demonstrating variable binding with partial XPath expressions
#[derive(Extract)]
struct ProductInfo {
    #[xee(xpath("//product[@id = $product_id]/name/text()"))]
    product_name: String,

    #[xee(xpath("//product[@id = $product_id]/price/text()"))]
    price: f64,

    #[xee(xpath("//product[@id = $product_id]/@category"))]
    category: String,

    #[xee(xpath("//product[@id = $product_id]/specs/spec[@type = $spec_type]/text()"))]
    spec_value: String,
}

/// A struct demonstrating variable binding with conditional logic
#[derive(Extract)]
struct ConditionalData {
    #[xee(xpath("$user_id"))]
    user_id: String,

    #[xee(xpath("if ($is_admin) then 'admin' else 'user'"))]
    role: String,

    #[xee(xpath("if ($has_permission) then 'granted' else 'denied'"))]
    permission: String,

    #[xee(xpath("$default_language"))]
    default_language: String,
}

fn main() {
    // Example 1: Basic variable binding
    println!("=== Example 1: Basic Variable Binding ===");

    let extractor1 = Extractor::default()
        .bind_value("user_name", "Alice Johnson")
        .bind_value("user_age", 28)
        .bind_value("is_active", true)
        .bind_value("score", 95.5);

    let user_profile: UserProfile = extractor1.extract_from_str("<root></root>").unwrap();

    println!("User Profile:");
    println!("  Name: {}", user_profile.user_name);
    println!("  Age: {}", user_profile.user_age);
    println!("  Active: {}", user_profile.is_active);
    println!("  Score: {}", user_profile.score);
    println!();

    // Example 2: Variable binding with partial XPath expressions
    println!("=== Example 2: Partial XPath with Variables ===");

    let product_xml = r#"
        <catalog>
            <product id="P001" category="electronics">
                <name>Laptop</name>
                <price>999.99</price>
                <specs>
                    <spec type="cpu">Intel i7</spec>
                    <spec type="ram">16GB</spec>
                    <spec type="storage">512GB SSD</spec>
                </specs>
            </product>
            <product id="P002" category="books">
                <name>Programming Guide</name>
                <price>29.99</price>
                <specs>
                    <spec type="pages">450</spec>
                    <spec type="format">Paperback</spec>
                </specs>
            </product>
        </catalog>
    "#;

    let extractor2 = Extractor::default()
        .bind_value("product_id", "P001")
        .bind_value("spec_type", "cpu");

    let product_info: ProductInfo = extractor2.extract_from_str(product_xml).unwrap();

    println!("Product Info:");
    println!("  Name: {}", product_info.product_name);
    println!("  Price: ${}", product_info.price);
    println!("  Category: {}", product_info.category);
    println!("  CPU Spec: {}", product_info.spec_value);
    println!();

    // Example 3: Conditional logic with variables
    println!("=== Example 3: Conditional Logic with Variables ===");

    let extractor3 = Extractor::default()
        .bind_value("user_id", "U123")
        .bind_value("is_admin", true)
        .bind_value("has_permission", false)
        .bind_value("default_language", "en");

    let conditional_data: ConditionalData = extractor3.extract_from_str("<root></root>").unwrap();

    println!("Conditional Data:");
    println!("  User ID: {}", conditional_data.user_id);
    println!("  Role: {}", conditional_data.role);
    println!("  Permission: {}", conditional_data.permission);
    println!("  Language: {}", conditional_data.default_language);
}
