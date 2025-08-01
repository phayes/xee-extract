# xee-extract

Declarative data extraction from large XML documents using Xpath. 

## Quick Start

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

## Field Attributes

### `#[xpath("expression")]`

Extract a single value using an XPath expression.

```rust
#[xpath("//title/text()")]
title: String,
```

### `#[extract("expression")]`

Extract a nested struct or vector of structs.

```rust
#[extract("//author")]
author: Author,

#[extract("//book")]
books: Vec<Book>,
```

### `#[xee(xml = "expression")]`

Extract raw XML content.

```rust
#[xml("//content")]
content: String,

#[xml("//metadata")]
metadata: Option<String>,
```

## Struct attributes

### `#[ns(name = "url")]`

Add namespaces for all xpath expressions. This has no effect if placed on a child struct (child structs inherit their parents namespaces when extracting via the parent). 

```rust
#[ns(
   atom = "http://www.w3.org/2005/Atom",
   meta = "http://example.org/Meta"
)]
struct MyStruct{
    #[xpath("atom:name/text()")]
    name: String,
}
```

### `#[context(expression)]`

Provide a custom context for the xpath expressions in this struct. By default top-level struct expressions are evaluated using the default root node, and child-structs are evaluated using the their extraction node as context. 

This can be useful when you have a struct that might be extracted as a child-node that is part of a larger structure, but also might be extracted on it's own.

```rust
#[context("(if self::entry then self else /entry)")]
struct Entry {
    #[xpath("id/text()")]
    id: String,
}
```
