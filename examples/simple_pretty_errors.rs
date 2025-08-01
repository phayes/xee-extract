use xee_extract::{Extractor, Extract, ExtractError};

#[derive(Extract, Debug)]
struct SimpleBook {
    #[xpath("//title/text()")]
    title: String,

    #[xpath("//author/text()")]
    author: String,

    #[xpath("//year/text()")]
    year: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Simple Pretty Error Handling Example ===\n");

    // Example 1: Successful extraction
    println!("1. Successful extraction:");
    let xml1 = r#"
        <book>
            <title>The Great Gatsby</title>
            <author>F. Scott Fitzgerald</author>
            <year>1925</year>
        </book>
    "#;

    let extractor = Extractor::new();
    let result1: Result<SimpleBook, ExtractError> = extractor.extract_one(xml1);
    
    match result1 {
        Ok(book) => {
            println!("Successfully extracted book:");
            println!("  Title: {}", book.title);
            println!("  Author: {}", book.author);
            println!("  Year: {:?}", book.year);
        }
        Err(error) => {
            println!("Unexpected error: {}", error);
        }
    }

    // Example 2: Missing required field
    println!("\n2. Error with missing required field:");
    let xml2 = r#"
        <book>
            <title>Sample Book</title>
            <year>2023</year>
        </book>
    "#;

    let result2: Result<SimpleBook, ExtractError> = extractor.extract_one(xml2);
    
    match result2 {
        Ok(_) => println!("Unexpected success!"),
        Err(error) => {
            println!("Error: {}", error);
        }
    }

    // Example 3: Invalid XML
    println!("\n3. Error with invalid XML:");
    let xml3 = r#"
        <book>
            <title>Sample Book</title>
            <author>John Doe</author>
            <unclosed>
        </book>
    "#;

    let result3: Result<SimpleBook, ExtractError> = extractor.extract_one(xml3);
    
    match result3 {
        Ok(_) => println!("Unexpected success!"),
        Err(error) => {
            println!("Error: {}", error);
        }
    }

    // Example 4: Empty XML
    println!("\n4. Error with empty XML:");
    let xml4 = "";

    let result4: Result<SimpleBook, ExtractError> = extractor.extract_one(xml4);
    
    match result4 {
        Ok(_) => println!("Unexpected success!"),
        Err(error) => {
            println!("Error: {}", error);
        }
    }

    Ok(())
} 