//! Example 2: Named Extractions
//! 
//! This example demonstrates how to use named extractions to extract data
//! from different XML structures using different extraction contexts.

use xee_extract::{Extractor, Extract};

/// A struct for basic user extraction
#[derive(Extract, Debug, PartialEq)]
struct BasicUser {
    #[xee(xpath("//name/text()"))]
    name: String,

    #[xee(xpath("//email/text()"))]
    email: String,
}

/// A struct for detailed user extraction
#[derive(Extract, Debug, PartialEq)]
struct DetailedUser {
    #[xee(xpath("//user/name/text()"))]
    name: String,

    #[xee(xpath("//user/email/text()"))]
    email: String,

    #[xee(xpath("//user/age/text()"))]
    age: Option<u32>,

    #[xee(xpath("//user/role/text()"))]
    role: Option<String>,
}

/// A struct for account-based user extraction
#[derive(Extract, Debug, PartialEq)]
struct AccountUser {
    #[xee(xpath("//account/owner/name/text()"))]
    name: String,

    #[xee(xpath("//account/owner/email/text()"))]
    email: String,

    #[xee(xpath("//account/owner/age/text()"))]
    age: Option<u32>,

    #[xee(xpath("//account/role/text()"))]
    role: Option<String>,
}

fn main() {
    // Example 1: Basic user extraction
    let basic_xml = r#"
        <user>
            <name>John Doe</name>
            <email>john@example.com</email>
        </user>
    "#;

    let extractor = Extractor::new();
    let user: BasicUser = extractor.extract_one(basic_xml).unwrap();

    println!("Basic user extraction:");
    println!("  Name: {}", user.name);
    println!("  Email: {}", user.email);
    println!();

    // Example 2: Detailed user extraction
    let detailed_xml = r#"
        <user>
            <name>Jane Smith</name>
            <email>jane@example.com</email>
            <age>28</age>
            <role>developer</role>
        </user>
    "#;

    let user: DetailedUser = extractor.extract_one(detailed_xml).unwrap();

    println!("Detailed user extraction:");
    println!("  Name: {}", user.name);
    println!("  Email: {}", user.email);
    println!("  Age: {:?}", user.age);
    println!("  Role: {:?}", user.role);
    println!();

    // Example 3: Account-based user extraction
    let account_xml = r#"
        <account>
            <id>ACC001</id>
            <owner>
                <name>Bob Wilson</name>
                <email>bob@example.com</email>
                <age>35</age>
            </owner>
            <role>admin</role>
            <balance>1000.00</balance>
        </account>
    "#;

    let user: AccountUser = extractor.extract_one(account_xml).unwrap();

    println!("Account-based user extraction:");
    println!("  Name: {}", user.name);
    println!("  Email: {}", user.email);
    println!("  Age: {:?}", user.age);
    println!("  Role: {:?}", user.role);
    println!();

    // Example 4: Demonstrating error handling for incompatible XML
    let incompatible_xml = r#"
        <unknown>
            <name>Unknown User</name>
            <email>unknown@example.com</email>
        </unknown>
    "#;

    let result = extractor.extract_one::<DetailedUser>(incompatible_xml);
    
    println!("Error handling for incompatible XML:");
    match result {
        Ok(user) => println!("  Unexpected success: {:?}", user),
        Err(e) => println!("  Expected error: {}", e),
    }
} 