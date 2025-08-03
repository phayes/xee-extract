//! Example 1: Basic Extraction
//!
//! This example demonstrates the fundamental usage of xee-extract for extracting
//! data from XML using XPath expressions.

use xee_extract::{Extract, Extractor};

/// A simple struct demonstrating basic field extraction
#[derive(Extract)]
struct Person {
    #[xee(xpath("//name"))]
    name: String,

    #[xee(xpath("//age"))]
    age: u32,

    #[xee(xpath("//email"))]
    email: Option<String>,

    #[xee(xpath("//hobbies/hobby"))]
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
            <email>john@example.com</email>
            <hobbies>
                <hobby>reading</hobby>
                <hobby>swimming</hobby>
                <hobby>coding</hobby>
            </hobbies>
        </person>
    "#;

    let extractor = Extractor::new();
    let person: Person = extractor.extract_from_str(person_xml).unwrap();

    println!("Person extracted:");
    println!("  Name: {}", person.name);
    println!("  Age: {}", person.age);
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

    // Example 3: Handling missing optional fields
    let person_without_email = r#"
        <person>
            <name>Jane Smith</name>
            <age>25</age>
            <hobbies>
                <hobby>painting</hobby>
            </hobbies>
        </person>
    "#;

    let person2: Person = extractor.extract_from_str(person_without_email).unwrap();

    println!("Person without email:");
    println!("  Name: {}", person2.name);
    println!("  Age: {}", person2.age);
    println!("  Email: {:?}", person2.email);
    println!("  Hobbies: {:?}", person2.hobbies);
}
