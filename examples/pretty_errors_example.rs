use xee_extract::{Extractor, Extract, ExtractError};

#[derive(Extract, Debug)]
struct Book {
    #[xee(xpath("title/text()"))]
    title: String,

    #[xee(xpath("author/text()"))]
    author: String,

    #[xee(xpath("year/text()"))]
    year: Option<String>,

    #[xee(xpath("genre/text()"))]
    genres: Vec<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Pretty Error Handling Example ===\n");

    // Example 1: Missing required field
    println!("1. Error with missing required field:");
    let xml1 = r#"
        <book>
            <title>Sample Book</title>
            <year>2023</year>
            <genre>Fiction</genre>
        </book>
    "#;

    let extractor = Extractor::new();
    let result1: Result<Book, ExtractError> = extractor.extract_one(xml1);
    
    match result1 {
        Ok(_) => println!("Unexpected success!"),
        Err(error) => {
            println!("Error: {}", error);
            println!("---");
        }
    }

    // Example 2: Invalid XML
    println!("\n2. Error with invalid XML:");
    let xml2 = r#"
        <book>
            <title>Sample Book</title>
            <author>John Doe</author>
            <unclosed>
        </book>
    "#;

    let result2: Result<Book, ExtractError> = extractor.extract_one(xml2);
    
    match result2 {
        Ok(_) => println!("Unexpected success!"),
        Err(error) => {
            println!("Error: {}", error);
            println!("---");
        }
    }

    // Example 3: Deserialization error
    println!("\n3. Error with deserialization (invalid number):");
    let xml3 = r#"
        <book>
            <title>Sample Book</title>
            <author>John Doe</author>
            <year>not_a_number</year>
            <genre>Fiction</genre>
        </book>
    "#;

    let result3: Result<Book, ExtractError> = extractor.extract_one(xml3);
    
    match result3 {
        Ok(_) => println!("Unexpected success!"),
        Err(error) => {
            println!("Error: {}", error);
            println!("---");
        }
    }

    // Example 4: Invalid XPath expression
    println!("\n4. Error with invalid XPath expression:");
    #[derive(Extract, Debug)]
    struct InvalidXPathBook {
        #[xee(xpath("title/text()"))]
        _title: String,
        
        // This XPath is invalid
        #[xee(xpath("invalid xpath ["))]
        _invalid: String,
    }

    let xml4 = r#"
        <book>
            <title>Sample Book</title>
        </book>
    "#;

    let result4: Result<InvalidXPathBook, ExtractError> = extractor.extract_one(xml4);
    
    match result4 {
        Ok(_) => println!("Unexpected success!"),
        Err(error) => {
            println!("Error: {}", error);
            println!("---");
        }
    }

    // Example 5: Empty XML
    println!("\n5. Error with empty XML:");
    let xml5 = "";

    let result5: Result<Book, ExtractError> = extractor.extract_one(xml5);
    
    match result5 {
        Ok(_) => println!("Unexpected success!"),
        Err(error) => {
            println!("Error: {}", error);
            println!("---");
        }
    }

    // Example 6: Successful extraction
    println!("\n6. Successful extraction:");
    let xml6 = r#"
        <book>
            <title>The Great Gatsby</title>
            <author>F. Scott Fitzgerald</author>
            <year>1925</year>
            <genre>Fiction</genre>
            <genre>Classic</genre>
        </book>
    "#;

    let result6: Result<Book, ExtractError> = extractor.extract_one(xml6);
    
    match result6 {
        Ok(book) => {
            println!("Successfully extracted book:");
            println!("  Title: {}", book.title);
            println!("  Author: {}", book.author);
            println!("  Year: {:?}", book.year);
            println!("  Genres: {:?}", book.genres);
        }
        Err(error) => {
            println!("Unexpected error: {}", error);
        }
    }

    Ok(())
} 