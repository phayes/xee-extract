use xee_extract::{Extractor, Extract};

#[derive(Extract, Debug, PartialEq)]
#[xee(default_ns("http://www.w3.org/2005/Atom"))]
#[xee(ns(
    dc = "http://purl.org/dc/elements/1.1/"
))]
#[xee(context("(//entry)[1]"))]
struct Entry {
    #[xee(xpath("id/text()"))]
    id: String,

    #[xee(xpath("title/text()"))]
    title: String,

    #[xee(xpath("dc:creator/text()"))]
    creator: Option<String>,

    #[xee(extract("author"))]
    author: Author,
}

#[derive(Extract, Debug, PartialEq)]
#[xee(default_ns("http://www.w3.org/2005/Atom"))]
#[xee(ns(
    dc = "http://purl.org/dc/elements/1.1/"
))]
struct Author {
    #[xee(xpath("name/text()"))]
    name: String,

    #[xee(xpath("dc:email/text()"))]
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