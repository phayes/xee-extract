use xee_extract::{Extractor, Extract};

#[derive(Extract, Debug, PartialEq)]
struct Article {
    #[xee(xpath("//article/@id"))]
    id: String,

    #[xee(xpath("//article/title/text()"))]
    title: String,

    #[xee(xml("//article/content"))]
    content: String,

    #[xee(xml("//article/metadata"))]
    metadata: Option<String>,

    #[xee(extract("//article/author"))]
    authors: Vec<Author>,
}

#[derive(Extract, Debug, PartialEq)]
struct Author {
    #[xee(xpath("name/text()"))]
    name: String,

    #[xee(xml("bio"))]
    bio: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let xml = r#"
        <article id="art-001">
            <title>Rust Programming Guide</title>
            <content>
                <section>
                    <h1>Introduction</h1>
                    <p>Rust is a systems programming language...</p>
                </section>
                <section>
                    <h1>Ownership</h1>
                    <p>One of Rust's key features is ownership...</p>
                </section>
            </content>
            <metadata>
                <created>2023-01-15</created>
                <updated>2023-02-20</updated>
                <tags>
                    <tag>programming</tag>
                    <tag>rust</tag>
                    <tag>systems</tag>
                </tags>
            </metadata>
            <author>
                <name>Alice Johnson</name>
                <bio>
                    <p>Alice is a senior software engineer with 10 years of experience.</p>
                    <p>She specializes in systems programming and Rust.</p>
                </bio>
            </author>
            <author>
                <name>Bob Smith</name>
                <bio>
                    <p>Bob is a technical writer and educator.</p>
                </bio>
            </author>
        </article>
    "#;

    let extractor = Extractor::new();
    let article: Article = extractor.extract_one(xml)?;

    println!("Article ID: {}", article.id);
    println!("Title: {}", article.title);
    println!("\nContent (raw XML):");
    println!("{}", article.content);

    if let Some(metadata) = &article.metadata {
        println!("\nMetadata (raw XML):");
        println!("{}", metadata);
    }

    println!("\nAuthors:");
    for author in &article.authors {
        println!("  - {} ({:?})", author.name, author.bio);
    }

    Ok(())
}
