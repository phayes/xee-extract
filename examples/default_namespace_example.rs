use xee_extract::{Extractor, Extract};

#[derive(Extract, Debug, PartialEq)]
#[default_ns("http://www.w3.org/2005/Atom")]
#[context("(//entry)[1]")]
struct Entry {
    #[xpath("id/text()")]
    id: String,

    #[xpath("title/text()")]
    title: String,

    #[xpath("category/@term")]
    category: Option<String>,

    #[extract("author")]
    author: Author,
}

#[derive(Extract, Debug, PartialEq)]
#[default_ns("http://www.w3.org/2005/Atom")]
struct Author {
    #[xpath("name/text()")]
    name: String,

    #[xpath("email/text()")]
    email: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let xml = r#"
        <feed xmlns="http://www.w3.org/2005/Atom">
            <entry>
                <id>123</id>
                <title>Sample Title</title>
                <category term="test"/>
                <author>
                    <name>John Doe</name>
                    <email>john@example.com</email>
                </author>
            </entry>
        </feed>
    "#;

    let extractor = Extractor::new();
    let entry: Entry = extractor.extract_one(xml)?;

    println!("ID: {}", entry.id);
    println!("Title: {}", entry.title);
    println!("Category: {:?}", entry.category);
    println!("Author: {} ({:?})", entry.author.name, entry.author.email);

    Ok(())
} 