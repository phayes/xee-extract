use xee_extract::{Extractor, Extract};

#[derive(Extract, Debug, PartialEq)]
#[default_ns("http://www.w3.org/2005/Atom")]
#[ns(
    dc = "http://purl.org/dc/elements/1.1/"
)]
#[context("(//entry)[1]")]
struct Entry {
    #[xpath("id/text()")]
    id: String,

    #[xpath("title/text()")]
    title: String,

    #[xpath("dc:creator/text()")]
    creator: Option<String>,

    #[extract("author")]
    author: Author,
}

#[derive(Extract, Debug, PartialEq)]
#[default_ns("http://www.w3.org/2005/Atom")]
#[ns(
    dc = "http://purl.org/dc/elements/1.1/"
)]
struct Author {
    #[xpath("name/text()")]
    name: String,

    #[xpath("dc:email/text()")]
    email: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let xml = r#"
        <feed xmlns="http://www.w3.org/2005/Atom" 
              xmlns:dc="http://purl.org/dc/elements/1.1/">
            <entry>
                <id>123</id>
                <title>Sample Title</title>
                <dc:creator>Jane Smith</dc:creator>
                <author>
                    <name>John Doe</name>
                    <dc:email>john@example.com</dc:email>
                </author>
            </entry>
        </feed>
    "#;

    let extractor = Extractor::new();
    let entry: Entry = extractor.extract_one(xml)?;

    println!("ID: {}", entry.id);
    println!("Title: {}", entry.title);
    println!("Creator: {:?}", entry.creator);
    println!("Author: {} ({:?})", entry.author.name, entry.author.email);

    Ok(())
} 