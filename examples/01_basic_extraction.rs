//! Example 1: Basic Extraction
//!
//! This example demonstrates the fundamental usage of xee-extract for extracting
//! data from XML using XPath expressions.

use xee_extract::{Extract, Extractor};

fn default_nickname() -> String {
    "No nickname".to_string()
}

/// A simple struct demonstrating basic field extraction
#[derive(Extract)]
struct Person {
    #[xee(xpath("//name"))]
    name: String,

    #[xee(xpath("//age"))]
    age: u32,

    #[xee(xpath("//nickname"))]
    #[xee(default("default_nickname"))]
    nickname: String,

    #[xee(xpath("//email"))]
    email: Option<String>,

    #[xee(xpath("//hobbies/hobby"))]
    #[xee(default)]
    hobbies: Vec<String>,
}

/// A more complex struct with nested data
#[derive(Extract)]
struct Company {
    #[xee(xpath("//company/@id"))]
    id: String,

    #[xee(xpath("//company/name"))]
    name: String,

    #[xee(xpath("//company/employees/employee/name"))]
    employee_names: Vec<String>,

    #[xee(xpath("//company/address/city"))]
    city: Option<String>,
}

fn main() {
    // Example 1: Simple person extraction
    let person_xml = r#"
        <person>
            <name>John Doe</name>
            <age>30</age>
            <nickname>Johnny</nickname>
            <email>john@example.com</email>
            <hobbies>
                <hobby>reading</hobby>
                <hobby>swimming</hobby>
                <hobby>coding</hobby>
            </hobbies>
        </person>
    "#;

    let extractor = Extractor::default();
    let person: Person = extractor.extract_from_str(person_xml).unwrap();

    println!("Person extracted:");
    println!("  Name: {}", person.name);
    println!("  Age: {}", person.age);
    println!("  Nickname: {}", person.nickname);
    println!("  Email: {:?}", person.email);
    println!("  Hobbies: {:?}", person.hobbies);
    println!();

    // Example 2: Company with employees
    let company_xml = r#"
        <company id="C001">
            <name>Tech Corp</name>
            <employees>
                <employee>
                    <name>Alice Johnson</name>
                </employee>
                <employee>
                    <name>Bob Smith</name>
                </employee>
                <employee>
                    <name>Carol Davis</name>
                </employee>
            </employees>
            <address>
                <city>San Francisco</city>
            </address>
        </company>
    "#;

    let company: Company = extractor.extract_from_str(company_xml).unwrap();

    println!("Company extracted:");
    println!("  ID: {}", company.id);
    println!("  Name: {}", company.name);
    println!("  Employees: {:?}", company.employee_names);
    println!("  City: {:?}", company.city);
    println!();

    // Example 3: Handling missing data with defaults
    let person_with_defaults = r#"
        <person>
            <name>Jane Smith</name>
            <age>25</age>
        </person>
    "#;

    let person2: Person = extractor.extract_from_str(person_with_defaults).unwrap();

    println!("Person with defaults:");
    println!("  Name: {}", person2.name);
    println!("  Age: {}", person2.age);
    println!("  Nickname: {}", person2.nickname);
    println!("  Email: {:?}", person2.email);
    println!("  Hobbies: {:?}", person2.hobbies);
}
