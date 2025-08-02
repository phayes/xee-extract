//! Example 3: Custom ExtractValue
//! 
//! This example demonstrates how to implement custom ExtractValue for custom types
//! that don't implement FromStr or need custom parsing logic.

use xee_extract::{Extractor, Extract, ExtractValue, Error};
use xee_xpath::{Documents, Item};

/// Custom struct for CSV data that implements ExtractValue
#[derive(Debug, PartialEq)]
struct CSV {
    values: Vec<String>,
}

impl CSV {
    fn new(values: Vec<String>) -> Self {
        Self { values }
    }
}

/// Custom ExtractValue implementation for CSV
impl ExtractValue for CSV {
    fn extract_value(documents: &mut Documents, item: &Item) -> Result<Self, Error> {
        let s = match item.string_value(documents.xot()) {
            Ok(s) => s,
            Err(_) => return Ok(CSV::new(Vec::new())), // Return empty CSV for any string value error
        };
        
        // Handle empty string case
        if s.trim().is_empty() {
            return Ok(CSV::new(Vec::new()));
        }
        
        // Parse comma-separated values, trimming whitespace
        let values: Vec<String> = s
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty()) // Filter out empty strings
            .collect();
        
        Ok(CSV::new(values))
    }
}

/// Custom struct for coordinates that implements ExtractValue
#[derive(Debug, PartialEq)]
struct Coordinates {
    latitude: f64,
    longitude: f64,
}

impl Coordinates {
    fn new(latitude: f64, longitude: f64) -> Self {
        Self { latitude, longitude }
    }
}

/// Custom ExtractValue implementation for Coordinates
impl ExtractValue for Coordinates {
    fn extract_value(documents: &mut Documents, item: &Item) -> Result<Self, Error> {
        let s = item.string_value(documents.xot())?;
        
        // Parse "lat,lon" format
        let parts: Vec<&str> = s.split(',').collect();
        if parts.len() != 2 {
            return Err(Error::DeserializationError(
                format!("Invalid coordinates format: {}", s)
            ));
        }
        
        let lat = parts[0].trim().parse::<f64>()
            .map_err(|_| Error::DeserializationError(
                format!("Invalid latitude: {}", parts[0])
            ))?;
        
        let lon = parts[1].trim().parse::<f64>()
            .map_err(|_| Error::DeserializationError(
                format!("Invalid longitude: {}", parts[1])
            ))?;
        
        Ok(Coordinates::new(lat, lon))
    }
}

/// Custom struct for date range that implements ExtractValue
#[derive(Debug, PartialEq)]
struct DateRange {
    start_date: String,
    end_date: String,
}

impl DateRange {
    fn new(start_date: String, end_date: String) -> Self {
        Self { start_date, end_date }
    }
}

/// Custom ExtractValue implementation for DateRange
impl ExtractValue for DateRange {
    fn extract_value(documents: &mut Documents, item: &Item) -> Result<Self, Error> {
        let s = item.string_value(documents.xot())?;
        
        // Parse "start to end" format
        let parts: Vec<&str> = s.split(" to ").collect();
        if parts.len() != 2 {
            return Err(Error::DeserializationError(
                format!("Invalid date range format: {}", s)
            ));
        }
        
        Ok(DateRange::new(
            parts[0].trim().to_string(),
            parts[1].trim().to_string()
        ))
    }
}

/// Struct using custom ExtractValue implementations
#[derive(Extract, Debug, PartialEq)]
struct Product {
    #[xee(xpath("//name/text()"))]
    name: String,

    #[xee(xpath("//tags/text()"))]
    tags: Option<CSV>,

    #[xee(xpath("//location/text()"))]
    location: Option<Coordinates>,

    #[xee(xpath("//availability/text()"))]
    availability: Option<DateRange>,

    #[xee(xpath("//categories/text()"))]
    categories: Option<CSV>,
}

/// Struct for complex data with custom types
#[derive(Extract, Debug, PartialEq)]
struct Store {
    #[xee(xpath("//store/@id"))]
    id: String,

    #[xee(xpath("//store/name/text()"))]
    name: String,

    #[xee(xpath("//store/coordinates/text()"))]
    coordinates: Coordinates,

    #[xee(xpath("//store/hours/text()"))]
    hours: DateRange,
}

fn main() {
    // Example 1: Product with CSV tags and categories
    let product_xml = r#"
        <product>
            <name>Laptop Computer</name>
            <tags>electronics, computer, portable, high-performance</tags>
            <location>37.7749, -122.4194</location>
            <availability>2024-01-01 to 2024-12-31</availability>
            <categories>hardware, computing, mobile</categories>
        </product>
    "#;

    let extractor = Extractor::new();
    let product: Product = extractor.extract_one(product_xml).unwrap();

    println!("Product with custom ExtractValue types:");
    println!("  Name: {}", product.name);
    println!("  Tags: {:?}", product.tags.as_ref().map(|csv| &csv.values));
    println!("  Location: {:?}", product.location);
    println!("  Availability: {:?}", product.availability);
    println!("  Categories: {:?}", product.categories.as_ref().map(|csv| &csv.values));
    println!();

    // Example 2: Store with coordinates and hours
    let store_xml = r#"
        <store id="STORE001">
            <name>Tech Store</name>
            <coordinates>40.7128, -74.0060</coordinates>
            <hours>09:00 to 18:00</hours>
        </store>
    "#;

    let store: Store = extractor.extract_one(store_xml).unwrap();

    println!("Store with custom ExtractValue types:");
    println!("  ID: {}", store.id);
    println!("  Name: {}", store.name);
    println!("  Coordinates: ({}, {})", store.coordinates.latitude, store.coordinates.longitude);
    println!("  Hours: {} to {}", store.hours.start_date, store.hours.end_date);
    println!();

    // Example 3: Product with missing optional fields
    let minimal_xml = r#"
        <product>
            <name>Simple Product</name>
            <tags>basic, simple</tags>
            <categories>general</categories>
        </product>
    "#;

    let product: Product = extractor.extract_one(minimal_xml).unwrap();

    println!("Product with missing optional fields:");
    println!("  Name: {}", product.name);
    println!("  Tags: {:?}", product.tags.as_ref().map(|csv| &csv.values));
    println!("  Location: {:?}", product.location);
    println!("  Availability: {:?}", product.availability);
    println!("  Categories: {:?}", product.categories.as_ref().map(|csv| &csv.values));
    println!();

    // Example 4: Error handling for invalid CSV data
    let invalid_csv_xml = r#"
        <product>
            <name>Invalid Product</name>
            <tags></tags>
            <categories>valid, category</categories>
        </product>
    "#;

    let product: Product = extractor.extract_one(invalid_csv_xml).unwrap();

    println!("Product with empty CSV (filtered out):");
    println!("  Name: {}", product.name);
    println!("  Tags: {:?}", product.tags.as_ref().map(|csv| &csv.values));
    println!("  Categories: {:?}", product.categories.as_ref().map(|csv| &csv.values));
    println!();

    // Example 5: Error handling for invalid coordinates
    let invalid_coords_xml = r#"
        <product>
            <name>Invalid Coordinates</name>
            <tags>test</tags>
            <location>invalid, coordinates, format</location>
            <categories>test</categories>
        </product>
    "#;

    let result = extractor.extract_one::<Product>(invalid_coords_xml);
    
    println!("Error handling for invalid coordinates:");
    match result {
        Ok(product) => println!("  Unexpected success: {:?}", product),
        Err(e) => println!("  Expected error: {}", e),
    }

    // Example 6: Error handling for invalid date range
    let invalid_date_xml = r#"
        <product>
            <name>Invalid Date</name>
            <tags>test</tags>
            <availability>invalid date format</availability>
            <categories>test</categories>
        </product>
    "#;

    let result = extractor.extract_one::<Product>(invalid_date_xml);
    
    println!("Error handling for invalid date range:");
    match result {
        Ok(product) => println!("  Unexpected success: {:?}", product),
        Err(e) => println!("  Expected error: {}", e),
    }

    // Example 7: Complex CSV with various formats
    let complex_csv_xml = r#"
        <product>
            <name>Complex Product</name>
            <tags>tag1, tag2 , tag3,  tag4  , tag5</tags>
            <categories>category1,category2, category3 ,  category4</categories>
        </product>
    "#;

    let product: Product = extractor.extract_one(complex_csv_xml).unwrap();

    println!("Complex CSV with various whitespace:");
    println!("  Name: {}", product.name);
    println!("  Tags: {:?}", product.tags.as_ref().map(|csv| &csv.values));
    println!("  Categories: {:?}", product.categories.as_ref().map(|csv| &csv.values));
} 