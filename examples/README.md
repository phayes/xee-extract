# xee-extract Examples

This directory contains comprehensive examples demonstrating the various features and capabilities of the xee-extract library.

## Examples Overview

### 1. Basic Extraction (`01_basic_extraction.rs`)
Demonstrates fundamental usage of xee-extract for extracting data from XML using XPath expressions.
- Simple field extraction with `xpath` attribute
- Optional fields handling
- Vector fields for multiple elements
- Basic error handling

### 2. Named Extractions (`02_named_extractions.rs`)
Shows how to extract data from different XML structures using separate structs for different extraction contexts.
- Different structs for different XML formats
- Handling various XML structures
- Error handling for incompatible XML

### 3. Custom ExtractValue (`03_custom_extract_value.rs`)
Demonstrates how to implement custom types that work with xee-extract.
- Custom enums with `FromStr` implementation
- Custom structs for complex data types
- Automatic `ExtractValue` implementation via `FromStr`
- Error handling for invalid custom types

### 4. Namespaces (`04_namespaces.rs`)
Shows how to handle XML namespaces in xee-extract.
- Prefixed namespaces with `ns` attribute
- Default namespaces with `default_ns` attribute
- Multiple namespaces in the same struct
- Error handling for missing namespaces

### 5. Contexts (`05_contexts.rs`)
Demonstrates how to use context in xee-extract to set the starting point for XPath expressions.
- Simple context with specific elements
- Conditional context for different XML structures
- Position-based context
- Complex context logic with filters

### 6. Nested Structs (`06_nested_structs.rs`)
Shows how to use nested structs with the `extract` attribute for complex XML structures.
- Nested structs with `extract` attribute
- Multiple levels of nesting
- Vector of nested structs
- Error handling for missing nested elements

### 7. Raw XML (`07_raw_xml.rs`)
Demonstrates how to extract raw XML content using the `xml` attribute.
- Raw XML extraction with `xml` attribute
- HTML content embedded in XML
- Configuration with raw XML
- Mixed content handling

### 8. Binary Handling (`08_binary_handling.rs`)
Shows how to handle binary data including hex and base64 encoding.
- Base64 encoded binary data
- Hex encoded binary data
- Mixed binary data types
- Binary data with metadata
- Error handling for invalid binary data

## Running the Examples

To run a specific example:

```bash
cargo run --example 01_basic_extraction
cargo run --example 02_named_extractions
# ... and so on for each example
```

To run all examples:

```bash
cargo run --examples
```

## Key Features Demonstrated

### Attributes Used
- `#[xee(xpath("..."))]` - Basic XPath extraction
- `#[xee(extract("..."))]` - Nested struct extraction
- `#[xee(xml("..."))]` - Raw XML extraction
- `#[xee(ns(prefix = "uri"))]` - Namespace declaration
- `#[xee(default_ns("uri"))]` - Default namespace
- `#[xee(context("..."))]` - Context setting

### Data Types Supported
- `String` - Text content
- `Option<String>` - Optional text content
- `Vec<String>` - Multiple text elements
- `u32`, `f64` - Numeric types
- `Vec<u8>` - Binary data
- `Option<Vec<u8>>` - Optional binary data
- Custom types with `FromStr` implementation

### Error Handling
- Missing elements
- Invalid data types
- Malformed XML
- Invalid binary data
- Missing namespaces
- Context not found

## Common Patterns

### Basic Extraction
```rust
#[derive(Extract, Debug, PartialEq)]
struct MyStruct {
    #[xee(xpath("//title/text()"))]
    title: String,
    
    #[xee(xpath("//description/text()"))]
    description: Option<String>,
}
```

### Nested Structs
```rust
#[derive(Extract, Debug, PartialEq)]
struct Parent {
    #[xee(xpath("@id"))]
    id: String,
    
    #[xee(extract("child"))]
    child: Child,
}

#[derive(Extract, Debug, PartialEq)]
struct Child {
    #[xee(xpath("name/text()"))]
    name: String,
}
```

### Namespaces
```rust
#[derive(Extract, Debug, PartialEq)]
#[xee(ns(atom = "http://www.w3.org/2005/Atom"))]
struct AtomFeed {
    #[xee(xpath("//atom:feed/atom:title/text()"))]
    title: String,
}
```

### Context
```rust
#[derive(Extract, Debug, PartialEq)]
#[xee(context("//book"))]
struct Book {
    #[xee(xpath("@id"))]
    id: String,
    
    #[xee(xpath("title/text()"))]
    title: String,
}
```

### Binary Data
```rust
#[derive(Extract, Debug, PartialEq)]
#[xee(ns(xs = "http://www.w3.org/2001/XMLSchema"))]
struct BinaryData {
    #[xee(xpath("string(//data/text()) cast as xs:base64Binary"))]
    data: Vec<u8>,
}
```

## Best Practices

1. **Use descriptive field names** that match your XML structure
2. **Handle optional fields** with `Option<T>` when elements might not exist
3. **Use vectors** for multiple elements of the same type
4. **Implement `FromStr`** for custom types to get automatic `ExtractValue`
5. **Use namespaces** when working with XML that has namespace declarations
6. **Set appropriate context** to simplify XPath expressions
7. **Handle errors gracefully** by checking extraction results
8. **Use raw XML extraction** when you need to preserve XML structure

## Error Messages

xee-extract provides detailed error messages that include:
- The specific field that failed to extract
- The XPath expression that was used
- The line and column where the error occurred
- Context around the error location
- Suggestions for fixing the issue

This makes debugging extraction issues much easier and helps you quickly identify and fix problems in your XML extraction code. 