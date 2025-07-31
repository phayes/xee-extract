use xee_extract::{XeeExtract, XeeExtractDeserialize, Extractor, Error};

#[derive(XeeExtract, Debug)]
//#[xpath(ns(
//    atom = "http://www.w3.org/2005/Atom",
//    nlm = "https://id.nlm.nih.gov/datmm/",
//    meta = "http://example.org/Meta"
//))]
//#[xpath(var(
//    baseurl = "if ($env = 'production') then 'https://prod.api.org' else 'https://dev.api.org'",
//    short_id = "tokenize(atom:id, ':')[last()]"
//))]
struct Entry {
    #[xpath("atom:id/text()")]
    id: String,

    #[xpath("$short_id")]
    short_id: String,

    #[xpath("if (exists(atom:subtitle)) then atom:subtitle else atom:title")]
    title: String,

    //#[xpath("atom:author")]
    //authors: Vec<Author>,

    #[xpath("concat($baseurl, '/entry/', $short_id)")]
    url: Option<String>,

    //#[xpath("//nlm:article-meta")]
    //metadata: Metadata,

    #[xpath("atom:category/@term")]
    category: Option<String>,
}

#[derive(XeeExtract, Debug)]
//#[xpath(ns(atom = "http://www.w3.org/2005/Atom"))]
struct Author {
    #[xpath("atom:name/text()")]
    name: String,

    #[xpath("atom:uri/text()")]
    homepage: Option<String>,
}

#[derive(XeeExtract, Debug)]
//#[xpath(ns(
//    nlm = "https://id.nlm.nih.gov/datmm/",
//))]
struct Metadata {
    #[xpath("nlm:fpage/text()")]
    first_page: Option<String>,

    #[xpath("nlm:lpage/text()")]
    last_page: Option<String>,

    #[xpath("nlm:pub-id[@pub-id-type='doi'][1]/text()")]
    doi: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let xml = std::fs::read_to_string("examples/entry.xml")?;
    let extractor = Extractor::new().with_variable("env", "production");

    // Extract a single struct from the XML document
    let entry: Entry = extractor.extract_one(&xml)?;
    
    println!("Extracted Entry:");
    println!("  ID: {}", entry.id);
    println!("  Short ID: {}", entry.short_id);
    println!("  Title: {}", entry.title);
    println!("  URL: {:?}", entry.url);
    println!("  Category: {:?}", entry.category);
    //println!("  Authors:");
    //for author in &entry.authors {
    //    println!("    - {} ({:?})", author.name, author.homepage);
    //}
    //println!("  Metadata:");
    //println!("    - First page: {:?}", entry.metadata.first_page);
    //println!("    - Last page: {:?}", entry.metadata.last_page);
    //println!("    - DOI: {:?}", entry.metadata.doi);

    Ok(())
} 