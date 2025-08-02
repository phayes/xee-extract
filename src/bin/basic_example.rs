use xee_extract::{Extractor, Extract};

#[derive(Extract, Debug, PartialEq)]
struct SimpleStruct {
    #[xee(xpath("//id/text()"))]
    id: String,

    #[xee(xpath("//title/text()"))]
    title: String,

    #[xee(xpath("//category/@term"))]
    category: Option<String>,
}

#[derive(Extract, Debug, PartialEq)]
struct ComplexStruct {
    #[xee(xpath("//id/text()"))]
    id: String,

    #[xee(xpath("//title/text()"))]
    title: String,

    #[xee(xpath("//subtitle/text()"))]
    subtitle: Option<String>,

    #[xee(xpath("//category/@term"))]
    category: Option<String>,

    #[xee(xpath("//tags/tag/text()"))]
    tags: Vec<String>,
}

#[derive(Extract, Debug, PartialEq)]
struct NestedStruct {
    #[xee(xpath("//id/text()"))]
    id: String,

    #[xee(xpath("//author/name/text()"))]
    author_name: String,

    #[xee(xpath("//author/email/text()"))]
    author_email: Option<String>,
}

fn main() {
    let xml = r#"
        <entry>
            <id>123</id>
            <title>Sample Title</title>
            <category term="test"/>
        </entry>
    "#;

    let extractor = Extractor::new();
    let result: SimpleStruct = extractor.extract_one(xml).expect("Extraction failed");

    println!("Extracted SimpleStruct:");
    println!("ID: {}", result.id);
    println!("Title: {}", result.title);
    println!("Category: {:?}", result.category);
}