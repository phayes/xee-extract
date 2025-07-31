use xee_extract::{XeeExtract, XeeExtractDeserialize, Extractor};

#[derive(XeeExtract, Debug)]
struct SimpleEntry {
    #[xpath("//id/text()")]
    id: String,

    #[xpath("//title/text()")]
    title: String,

    #[xpath("//category/@term")]
    category: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let xml = r#"
        <entry>
            <id>123</id>
            <title>Sample Title</title>
            <category term="test"/>
        </entry>
    "#;
    
    println!("XML: {}", xml);
    
    let extractor = Extractor::new();
    let entry: SimpleEntry = extractor.extract_one(xml)?;
    
    println!("Extracted Entry:");
    println!("  ID: {}", entry.id);
    println!("  Title: {}", entry.title);
    println!("  Category: {:?}", entry.category);

    Ok(())
} 