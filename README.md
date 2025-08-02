# xee-extract

Declarative data extraction from large XML documents using Xpath.

## Quick Start

```rust
use xee_extract::{Extractor, Extract};

#[derive(Extract)]
struct SimpleEntry {
    #[xee(xpath("//id/text()"))]
    id: String,

    #[xee(xpath("//title/text()"))]
    title: String,

    #[xee(xpath("//category/@term"))]
    category: Option<String>,

    #[xee(extract("//author"))]
    author: Author,
}

#[derive(Extract)]
struct Author {
    #[xee(xpath("name/text()"))]
    name: String,

    #[xee(xpath("email/text()"))]
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

### `#[xee(xpath("expression"))]`

Extract a single value using an XPath expression.

```rust
#[derive(xee_extract::Extract)]
struct Foo {
  #[xee(xpath("//title/text()"))]
  title: String,
}

```

### `#[xee(extract("expression"))]`

Extract a nested struct or vector of structs.

```rust
#[derive(xee_extract::Extract)]
struct Foo {
    #[xee(extract("//author"))]
    author: Author,
    
    #[xee(extract("//book"))]
    books: Vec<Book>,
}
```

### `#[xee(xml("expression"))]`

Extract raw XML content.

```rust
#[derive(xee_extract::Extract)]
struct Foo {
    #[xee(xml("//content"))]
    content: String,
    
    #[xee(xml("//metadata"))]
    metadata: Option<String>,
}
```

## Struct attributes

### `#[xee(context("expression"))]`

Provide a custom context for the xpath expressions in this struct. By default top-level struct expressions are evaluated using the default root node, and child-structs are evaluated using the their extraction node as context. 

This can be useful when you have a struct that might be extracted as a child-node that is part of a larger structure, but also might be extracted on it's own.

```rust
#[xee(context("(if self::entry then . else /entry)"))]
struct Entry {
    #[xee(xpath("id/text()"))]
    id: String,
}
```

### `#[xee(ns(name = "url"))]`

Add namespaces for all xpath expressions. This has no effect if placed on a child struct (child structs inherit their parents namespaces when extracting via the parent). 

```rust
#[xee(ns(atom = "http://www.w3.org/2005/Atom")]
#[xee(ns(meta = "http://example.org/Meta")]
struct Foo {
    #[xee(xpath("atom:name/text()"))]
    name: String,
}
```

### `#[xee(default_ns(name = "url"))]`

Set the default namespace for xpath queries.

```rust
#[xee(default_ns(atom = "http://www.w3.org/2005/Atom"))]
struct Foo {
    #[xee(xpath("name/text()"))]
    name: String,
}
```


## Custom Value Extraction

`ExtractValue` controls how individual field values are deserialized.
Any type that implements `FromStr` works out of the box, but you can
provide custom parsing by implementing this trait yourself.

### Example: parsing a comma separated list

```rust
use xee_extract::{Extract, Extractor, ExtractValue, Error};
use xee_xpath::{Documents, Item};

struct CsvTags(Vec<String>);

impl ExtractValue for CsvTags {
    fn extract_value(documents: &mut Documents, item: &Item) -> Result<Self, Error> {
        let s = item.string_value(documents.xot())?;
        Ok(CsvTags(
            s.split(',')
                .map(|s| s.trim().to_string())
                .collect(),
        ))
    }
}

#[derive(Extract)]
struct TaggedEntry {
    #[xee(xpath("//tags/text()"))]
    tags: CsvTags,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let xml = r#"<entry><tags>alpha, beta, gamma</tags></entry>"#;
    let extractor = Extractor::new();
    let entry: TaggedEntry = extractor.extract_one(xml)?;
    assert_eq!(entry.tags.0, vec!["alpha", "beta", "gamma"]);
    Ok(())
}
```

## Named Extractions

Sometimes a single struct needs to support multiple XML formats.  Each
`#[xee(...)]` attribute can take an optional second string argument that
associates it with a named extraction.  When using
`Extractor::named("nlm")`, only the attributes tagged with that name are
applied; attributes without a name form the default extraction used by
`Extractor::default()`.

```rust
use xee_extract::{Extractor, Extract};

#[derive(Extract)]
#[xee(ns(atom = "http://www.w3.org/2005/Atom"))]                // default
#[xee(ns(nlm = "https://id.nlm.nih.gov/datmm/", "nlm"))]       // named
struct Entry {
    #[xee(xpath("//atom:id/text()"))]                          // default
    #[xee(xpath("//nlm:id/text()", "nlm"))]                   // named
    id: String,

    #[xee(xpath("//atom:title/text()"))]
    #[xee(xpath("//nlm:title/text()", "nlm"))]
    title: String,
}

// Parse Atom
let atom: Entry = Extractor::default().extract_one(atom_xml)?;
// Parse NLM using the named extraction
let nlm: Entry = Extractor::named("nlm").extract_one(nlm_xml)?;
```

This mechanism works for other struct-level attributes like `context` and
`default_ns`, enabling a single type to handle multiple extraction
configurations.
