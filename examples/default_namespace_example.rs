use xee_extract::{Extractor, Extract};

#[derive(Extract, Debug, PartialEq)]
#[xee(default_ns("http://www.w3.org/2005/Atom"))]
#[xee(context("(//entry)[1]"))]
struct Entry {
    #[xee(xpath("id/text()"))]
    id: String,

    #[xee(xpath("title/text()"))]
    title: String,

    #[xee(xpath("category/@term"))]
    category: Option<String>,

    #[xee(extract("author"))]
    author: Author,
}

#[derive(Extract, Debug, PartialEq)]
#[xee(default_ns("http://www.w3.org/2005/Atom"))]
struct Author {
    #[xee(xpath("name/text()"))]
    name: String,

    #[xee(xpath("email/text()"))]
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