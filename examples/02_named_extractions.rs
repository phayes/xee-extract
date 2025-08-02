//! Example 2: Named Extractions
//! 
//! This example demonstrates how to use named extractions to extract the same
//! struct from different XML structures using different extraction configurations.

use xee_extract::{Extractor, Extract};

/// A struct that can be extracted from different XML structures
/// using named extractions with different namespaces and XPath expressions
#[derive(Extract, Debug, PartialEq)]
#[xee(ns(atom = "http://www.w3.org/2005/Atom"))]                // default namespace
#[xee(ns(nlm = "https://id.nlm.nih.gov/datmm/", "foo"))]        // named extraction "foo"
#[xee(ns(atom = "http://www.w3.org/2005/Atom", "bar"))]         // named extraction "bar"
#[xee(context("//atom:entry", "bar"))]                          // named extraction "bar"
struct Entry {
    #[xee(xpath("//atom:id/text()"))]                          // default
    #[xee(xpath("//nlm:id/text()", "foo"))]                   
    #[xee(xpath("atom:id/text()", "bar"))]                 
    id: String,

    #[xee(xpath("//atom:title/text()"))]                       // default
    #[xee(xpath("//nlm:title/text()", "foo"))]                
    #[xee(xpath("atom:title/text()", "bar"))]              
    title: String,

    #[xee(xpath("//atom:author/atom:name/text()"))]           
    #[xee(xpath("//nlm:contrib-group/nlm:contrib/nlm:name/text()", "foo"))] 
    #[xee(xpath("atom:author/atom:name/text()", "bar"))]  
    author: Option<String>,
}

fn main() {
    // Example 1: Default Atom extraction
    let atom_xml = r#"
        <entry xmlns="http://www.w3.org/2005/Atom">
            <id>urn:uuid:12345678-1234-1234-1234-123456789abc</id>
            <title>Atom Title</title>
            <author>
                <name>Alice Johnson</name>
            </author>
        </entry>
    "#;

    let extractor = Extractor::new(); // or Extractor::default()
    let entry: Entry = extractor.extract_one(atom_xml).unwrap();

    println!("Default Atom extraction:");
    println!("  ID: {}", entry.id);
    println!("  Title: {}", entry.title);
    println!("  Author: {:?}", entry.author);
    println!();

    // Example 2: Named NLM extraction
    let nlm_xml = r#"
        <article xmlns:nlm="https://id.nlm.nih.gov/datmm/">
            <nlm:id>abc123</nlm:id>
            <nlm:title>NLM Title</nlm:title>
            <nlm:contrib-group>
                <nlm:contrib>
                    <nlm:name>Bob Smith</nlm:name>
                </nlm:contrib>
            </nlm:contrib-group>
        </article>
    "#;

    let extractor = Extractor::named("foo");
    let entry: Entry = extractor.extract_one(nlm_xml).unwrap();

    println!("Named NLM extraction:");
    println!("  ID: {}", entry.id);
    println!("  Title: {}", entry.title);
    println!("  Author: {:?}", entry.author);
    println!();

    // Example 3: Context-based extraction
    let context_xml = r#"
        <feed xmlns:atom="http://www.w3.org/2005/Atom">
            <atom:entry>
                <atom:id>c456</atom:id>
                <atom:title>Context Title</atom:title>
                <atom:author>
                    <atom:name>Carol Davis</atom:name>
                </atom:author>
            </atom:entry>
        </feed>
    "#;

    let extractor = Extractor::named("bar");
    let entry: Entry = extractor.extract_one(context_xml).unwrap();

    println!("Context-based extraction:");
    println!("  ID: {}", entry.id);
    println!("  Title: {}", entry.title);
    println!("  Author: {:?}", entry.author);
    println!();

    // Example 4: Demonstrating error handling for wrong extraction
    let wrong_xml = r#"
        <unknown>
            <id>wrong-id</id>
            <title>Wrong Title</title>
        </unknown>
    "#;

    let extractor = Extractor::named("foo");
    let result = extractor.extract_one::<Entry>(wrong_xml);
    
    println!("Error handling for incompatible XML:");
    match result {
        Ok(entry) => println!("  Unexpected success: {:?}", entry),
        Err(e) => println!("  Expected error: {}", e),
    }

    // Example 5: Demonstrating that default extraction works with Atom XML
    let atom_xml_2 = r#"
        <entry xmlns="http://www.w3.org/2005/Atom">
            <id>urn:uuid:87654321-4321-4321-4321-cba987654321</id>
            <title>Another Atom Title</title>
            <author>
                <name>David Wilson</name>
            </author>
        </entry>
    "#;

    let extractor = Extractor::default(); // explicitly use default
    let entry: Entry = extractor.extract_one(atom_xml_2).unwrap();

    println!("Default extraction (explicit):");
    println!("  ID: {}", entry.id);
    println!("  Title: {}", entry.title);
    println!("  Author: {:?}", entry.author);
    println!();

    // Example 6: Demonstrating that named extractions can have different structures
    let nlm_xml_2 = r#"
        <article xmlns:nlm="https://id.nlm.nih.gov/datmm/">
            <nlm:id>xyz789</nlm:id>
            <nlm:title>Another NLM Title</nlm:title>
            <!-- No author in this example -->
        </article>
    "#;

    let extractor = Extractor::named("foo");
    let entry: Entry = extractor.extract_one(nlm_xml_2).unwrap();

    println!("Named extraction with missing optional field:");
    println!("  ID: {}", entry.id);
    println!("  Title: {}", entry.title);
    println!("  Author: {:?}", entry.author);
} 
