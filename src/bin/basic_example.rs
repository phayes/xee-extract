use xee_extract::{Extractor, Extract};

#[derive(Extract, Debug)]
#[xee(ns(atom = "http://www.w3.org/2005/Atom"))]                 // Default extraction
#[xee(ns(nlm = "https://id.nlm.nih.gov/datmm/", "extract1"))]     // "extract1" extraction
struct Entry {
    #[xee(xpath("//atom:id/text()"))]         // Default extract
    #[xee(xpath("//nlm:id/text()", "extract1"))]     // "extract1" extract
    id: String,

    #[xee(xpath("//atom:title/text()"))]      // Default extract
    #[xee(xpath("//nlm:title/text()", "extract1"))]  // "extract1" extract
    title: String,

    #[xee(xpath("//atom:author/atom:name/text()"))]  // Default extract
    #[xee(xpath("//nlm:contrib-group/nlm:contrib/nlm:name/text()", "extract1"))]  // "extract1" extract
    author: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Example XML for Atom format
    let atom_xml = r#"
    <entry xmlns:atom="http://www.w3.org/2005/Atom">
        <atom:id>urn:uuid:12345678-1234-1234-1234-123456789abc</atom:id>
        <atom:title>Example Atom Entry</atom:title>
        <atom:author>
            <atom:name>John Doe</atom:name>
        </atom:author>
    </entry>
    "#;

    // Example XML for NLM format
    let nlm_xml = r#"
    <article xmlns:nlm="https://id.nlm.nih.gov/datmm/">
        <nlm:id>12345</nlm:id>
        <nlm:title>Example NLM Article</nlm:title>
        <nlm:contrib-group>
            <nlm:contrib>
                <nlm:name>Jane Smith</nlm:name>
            </nlm:contrib>
        </nlm:contrib-group>
    </article>
    "#;

    // Extract using default extractor
    println!("Default extraction:");
    let extractor = Extractor::default();
    let entry: Entry = extractor.extract_one(atom_xml).map_err(|e| e.message())?;

    println!("  ID: {}", entry.id);
    println!("  Title: {}", entry.title);
    println!("  Author: {:?}", entry.author);

    // Extract using named extractor
    println!("\nNamed extraction (extract1):");
    let extractor = Extractor::named("extract1");
    let entry: Entry = extractor.extract_one(nlm_xml).map_err(|e| e.message())?;
    
    println!("  ID: {}", entry.id);
    println!("  Title: {}", entry.title);
    println!("  Author: {:?}", entry.author);

    Ok(())
} 