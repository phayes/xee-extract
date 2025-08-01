use xee_extract::{XeeExtract, Extractor};

#[derive(XeeExtract, Debug, PartialEq)]
struct Library {
    #[xpath("//library/@name")]
    name: String,

    #[extract("//library/books/book")]
    books: Vec<Book>,
}

#[derive(XeeExtract, Debug, PartialEq)]
struct Book {
    #[xpath("title/text()")]
    title: String,

    #[xpath("author/text()")]
    author: String,

    #[xpath("year/text()")]
    year: Option<i32>,

    #[xpath("genre/text()")]
    genres: Vec<String>,
}

#[derive(XeeExtract, Debug, PartialEq)]
struct SimpleBook {
    #[xpath("title/text()")]
    title: String,

    #[xpath("author/text()")]
    author: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let xml = r#"
        <library name="My Library">
            <books>
                <book>
                    <title>The Rust Programming Language</title>
                    <author>Steve Klabnik</author>
                    <year>2018</year>
                    <genre>Programming</genre>
                    <genre>Reference</genre>
                </book>
                <book>
                    <title>Programming Rust</title>
                    <author>Jim Blandy</author>
                    <year>2021</year>
                    <genre>Programming</genre>
                </book>
                <book>
                    <title>Rust in Action</title>
                    <author>Tim McNamara</author>
                    <genre>Programming</genre>
                </book>
            </books>
        </library>
    "#;
    
    let extractor = Extractor::new();
    let library: Library = extractor.extract_one(xml)?;
    
    println!("Library: {}", library.name);
    println!("Books:");
    for book in &library.books {
        println!("  - {} by {} ({:?})", book.title, book.author, book.year);
        println!("    Genres: {:?}", book.genres);
    }

    Ok(())
} 