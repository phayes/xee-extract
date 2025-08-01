# xee-extract

A powerful Rust crate for XPath-driven XML data extraction using the Xee engine. This crate provides a procedural macro `Extract` that allows you to deserialize XML documents into Rust structs using XPath expressions.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
xee-extract = "0.1.0"
```

## Quick Start

Here's a simple example to get you started:

```rust
use xee_extract::{Extractor, Extract};

#[derive(Extract, Debug, PartialEq)]
struct SimpleEntry {
    #[xpath("//id/text()")]
    id: String,

    #[xpath("//title/text()")]
    title: String,

    #[xpath("//category/@term")]
    category: Option<String>,

    #[extract("//author")]
    author: Author,
}

#[derive(Extract, Debug, PartialEq)]
struct Author {
    #[xpath("name/text()")]
    name: String,

    #[xpath("email/text()")]
    email: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let xml = r#"
        <entry>
            <id>123</id>
            <title>Sample Title</title>
            <category term="test"/>
            <author>
                <name>John Doe</name>
                <email>john@example.com</email>
            </author>
        </entry>
    "#;

    let extractor = Extractor::new();
    let entry: SimpleEntry = extractor.extract_one(xml)?;

    println!("ID: {}", entry.id);
    println!("Title: {}", entry.title);
    println!("Category: {:?}", entry.category);
    println!("Author: {} ({:?})", entry.author.name, entry.author.email);

    Ok(())
}
```

## Attributes

### `#[xpath(expression)]`

Extract a single value using an XPath expression:

```rust
#[xpath("//title/text()")]
title: String,

#[xpath("//category/@term")]
category: Option<String>,
```

### `#[extract(expression)]`

Extract a nested struct or vector of structs:

```rust
#[extract("//author")]
author: Author,

#[extract("//book")]
books: Vec<Book>,
```

### `#[xml(expression)]`

Extract raw XML content:

```rust
#[xml("//content")]
content: String,

#[xml("//metadata")]
metadata: Option<String>,
```

## Error Handling

The crate provides two levels of error handling:

### Basic Error Handling

For simple error handling, use the standard `extract_one` method:

```rust
let result: Result<SimpleEntry, Error> = extractor.extract_one(xml);
match result {
    Ok(entry) => println!("Success: {:?}", entry),
    Err(error) => eprintln!("Error: {}", error),
}
```

### Pretty Error Handling

For user-friendly error messages with XML context, use the `extract_one_pretty` method:

```rust
let result: Result<SimpleEntry, ExtractorError> = extractor.extract_one_pretty(xml);
match result {
    Ok(entry) => println!("Success: {:?}", entry),
    Err(error) => {
        println!("Pretty error: {}", error);
        // The error includes:
        // - Human-readable error messages
        // - XML context around the error location
        // - Line number information when available
        // - Additional context if provided
    }
}
```

The `ExtractorError` provides rich error information including:
- **XML Context**: Shows the relevant XML snippet around the error location
- **Line Numbers**: Indicates approximately where the error occurred
- **Error Types**: Distinguishes between XPath errors, XML parsing errors, and deserialization errors
- **Additional Context**: Allows you to add custom context information

### Error Types

- `InvalidXPath`: Invalid XPath expressions
- `DeserializationError`: Failed to convert XML values to Rust types
- `SpannedError`: XPath errors with source location information
- `XeeInterpreterError`: Low-level XPath interpreter errors
- `DocumentsError`: XML document parsing errors

## TODO - Showcase of what is not yet supported, but will be

 - namespace support
 - variable support
 - context support

```rust
use xee_extract::{Extractor, Extract};


// Provides namespace registration
#[xee_ns(
   atom = "http://www.w3.org/2005/Atom",
   nlm = "https://id.nlm.nih.gov/datmm/",
   meta = "http://example.org/Meta"
)]

// Variables
#[xee_var(
   baseurl = "if ($env = 'production') then 'https://prod.api.org' else 'https://dev.api.org'",
   short_id = "tokenize(atom:id, ':')[last()]"
)]

// Context for all xpaths
#[xee_context("(if self::entry then self else /entry)")]

#[derive(Extract, Debug)]
struct Entry {
    ...
}
