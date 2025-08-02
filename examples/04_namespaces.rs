//! Example 4: Namespaces
//! 
//! This example demonstrates how to handle XML namespaces in xee-extract,
//! including default namespaces and prefixed namespaces.

use xee_extract::{Extractor, Extract};

/// Struct for extracting Atom feed data with namespaces
#[derive(Extract)]
#[xee(ns(atom = "http://www.w3.org/2005/Atom"))]
struct AtomFeed {
    #[xee(xpath("//atom:feed/atom:title/text()"))]
    title: String,

    #[xee(xpath("//atom:feed/atom:subtitle/text()"))]
    subtitle: Option<String>,

    #[xee(xpath("//atom:feed/atom:entry/atom:title/text()"))]
    entry_titles: Vec<String>,

    #[xee(xpath("//atom:feed/atom:entry/atom:author/atom:name/text()"))]
    author_names: Vec<String>,
}

/// Struct for extracting RSS feed data with namespaces
#[derive(Extract)]
#[xee(ns(rss = "http://purl.org/rss/1.0/"))]
struct RSSFeed {
    #[xee(xpath("//rss:rss/rss:channel/rss:title/text()"))]
    title: String,

    #[xee(xpath("//rss:rss/rss:channel/rss:description/text()"))]
    description: Option<String>,

    #[xee(xpath("//rss:rss/rss:channel/rss:item/rss:title/text()"))]
    item_titles: Vec<String>,
}

/// Struct for extracting SOAP data with multiple namespaces
#[derive(Extract)]
#[xee(ns(soap = "http://schemas.xmlsoap.org/soap/envelope/"))]
#[xee(ns(xsd = "http://www.w3.org/2001/XMLSchema"))]
struct SOAPMessage {
    #[xee(xpath("//soap:Envelope/soap:Header/soap:Action/text()"))]
    action: String,

    #[xee(xpath("//soap:Envelope/soap:Body/xsd:string/text()"))]
    body_content: Option<String>,

    #[xee(xpath("//soap:Envelope/soap:Header/soap:MessageID/text()"))]
    message_id: Option<String>,
}

/// Struct for extracting data with default namespace
#[derive(Extract)]
#[xee(default_ns("http://example.com/default"))]
struct DefaultNamespaceData {
    #[xee(xpath("//root/title/text()"))]
    title: String,

    #[xee(xpath("//root/items/item/text()"))]
    items: Vec<String>,
}

fn main() {
    // Example 1: Atom feed with namespaces
    let atom_xml = r#"
        <feed xmlns="http://www.w3.org/2005/Atom">
            <title>My Blog</title>
            <subtitle>A personal blog about technology</subtitle>
            <entry>
                <title>Getting Started with Rust</title>
                <author>
                    <name>John Doe</name>
                </author>
            </entry>
            <entry>
                <title>Advanced XPath Techniques</title>
                <author>
                    <name>Jane Smith</name>
                </author>
            </entry>
        </feed>
    "#;

    let extractor = Extractor::new();
    let feed: AtomFeed = extractor.extract_one(atom_xml).unwrap();

    println!("Atom feed with namespaces:");
    println!("  Title: {}", feed.title);
    println!("  Subtitle: {:?}", feed.subtitle);
    println!("  Entry Titles: {:?}", feed.entry_titles);
    println!("  Author Names: {:?}", feed.author_names);
    println!();

    // Example 2: RSS feed with namespaces
    let rss_xml = r#"
        <rss xmlns="http://purl.org/rss/1.0/" version="2.0">
            <channel>
                <title>Tech News</title>
                <description>Latest technology news and updates</description>
                <item>
                    <title>New Programming Language Released</title>
                </item>
                <item>
                    <title>AI Breakthrough in Machine Learning</title>
                </item>
                <item>
                    <title>Web Development Trends 2024</title>
                </item>
            </channel>
        </rss>
    "#;

    let feed: RSSFeed = extractor.extract_one(rss_xml).unwrap();

    println!("RSS feed with namespaces:");
    println!("  Title: {}", feed.title);
    println!("  Description: {:?}", feed.description);
    println!("  Item Titles: {:?}", feed.item_titles);
    println!();

    // Example 3: SOAP message with multiple namespaces
    let soap_xml = r#"
        <soap:Envelope xmlns:soap="http://schemas.xmlsoap.org/soap/envelope/"
                       xmlns:xsd="http://www.w3.org/2001/XMLSchema">
            <soap:Header>
                <soap:Action>GetUserInfo</soap:Action>
                <soap:MessageID>msg-12345</soap:MessageID>
            </soap:Header>
            <soap:Body>
                <xsd:string>user123</xsd:string>
            </soap:Body>
        </soap:Envelope>
    "#;

    let message: SOAPMessage = extractor.extract_one(soap_xml).unwrap();

    println!("SOAP message with multiple namespaces:");
    println!("  Action: {}", message.action);
    println!("  Body Content: {:?}", message.body_content);
    println!("  Message ID: {:?}", message.message_id);
    println!();

    // Example 4: Data with default namespace
    let default_ns_xml = r#"
        <root xmlns="http://example.com/default">
            <title>Default Namespace Example</title>
            <items>
                <item>Item 1</item>
                <item>Item 2</item>
                <item>Item 3</item>
            </items>
        </root>
    "#;

    let data: DefaultNamespaceData = extractor.extract_one(default_ns_xml).unwrap();

    println!("Data with default namespace:");
    println!("  Title: {}", data.title);
    println!("  Items: {:?}", data.items);
    println!();

    // Example 5: Error handling for missing namespace
    let no_namespace_xml = r#"
        <feed>
            <title>No Namespace Feed</title>
            <entry>
                <title>This won't work without namespace</title>
            </entry>
        </feed>
    "#;

    let result = extractor.extract_one::<AtomFeed>(no_namespace_xml);
    
    println!("Error handling for missing namespace:");
    match result {
        Ok(_feed) => println!("  Unexpected success: Feed extracted"),
        Err(e) => println!("  Expected error: {}", e),
    }
} 