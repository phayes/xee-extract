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

    println!("XML: {}", xml);

    let extractor = Extractor::new();
    let entry: SimpleEntry = extractor.extract_one(xml)?;

    println!("Extracted Entry:");
    println!("  ID: {}", entry.id);
    println!("  Title: {}", entry.title);
    println!("  Category: {:?}", entry.category);
    println!("  Author: {} ({:?})", entry.author.name, entry.author.email);

    Ok(())
}
