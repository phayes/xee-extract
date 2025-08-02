use xee_extract::{Extractor, Extract};

#[derive(Extract, Debug, PartialEq)]
#[xee(ns(
    atom = "http://www.w3.org/2005/Atom",
    nlm = "https://id.nlm.nih.gov/datmm/",
    meta = "http://example.org/Meta"
))]
#[xee(context("(//entry)[1]"))]
struct Entry {
    #[xee(xpath("atom:id/text()"))]
    id: String,

    #[xee(xpath("atom:title/text()"))]
    title: String,

    #[xee(xpath("atom:category/@term"))]
    category: Option<String>,

    #[xee(extract("atom:author"))]
    author: Author,
}

#[derive(Extract, Debug, PartialEq)]
#[xee(ns(
    atom = "http://www.w3.org/2005/Atom",
    nlm = "https://id.nlm.nih.gov/datmm/",
    meta = "http://example.org/Meta"
))]
struct Author {
    #[xee(xpath("atom:name/text()"))]
    name: String,

    #[xee(xpath("atom:email/text()"))]
    email: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let xml = r#"
        <feed xmlns:atom="http://www.w3.org/2005/Atom">
            <entry>
                <atom:id>123</atom:id>
                <atom:title>Sample Title</atom:title>
                <atom:category term="test"/>
                <atom:author>
                    <atom:name>John Doe</atom:name>
                    <atom:email>john@example.com</atom:email>
                </atom:author>
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