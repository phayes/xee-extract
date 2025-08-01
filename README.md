# xee-extract

A powerful Rust crate for XPath-driven XML data extractioin using the Xee engine. This crate provides a procedural macro `XeeExtract` that allows you to deserialize XML documents into Rust structs using XPath expressions.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
xee-extract = "0.1.0"
```

## Quick Start

Here's a simple example to get you started:

```rust
use xee_extract::{Extractor, XeeExtract};

#[derive(XeeExtract, Debug, PartialEq)]
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

#[derive(XeeExtract, Debug, PartialEq)]
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

## TODO - Showcase of what is not yet supported, but will be

 - namespace support
 - variable support
 - context support

```rust
use xee_extract::{Extractor, XeeExtract};


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

#[derive(XeeExtract, Debug)]
struct Entry {
    ...
}
