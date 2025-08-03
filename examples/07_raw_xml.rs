//! Example 7: Raw XML Extraction
//!
//! This example demonstrates how to extract raw XML content using the xml attribute,
//! which allows you to capture the complete XML structure of elements.

use xee_extract::{Extract, Extractor};

/// Struct for extracting raw XML content
#[derive(Extract)]
struct RawXMLData {
    #[xee(xpath("//title"))]
    title: String,

    #[xee(xml("//content"))]
    content_xml: String,

    #[xee(xml("//metadata"))]
    metadata_xml: Option<String>,

    #[xee(xml("//comments/comment"))]
    comment_xmls: Vec<String>,
}

/// Struct for extracting HTML content embedded in XML
#[derive(Extract)]
struct HTMLContent {
    #[xee(xpath("//title"))]
    title: String,

    #[xee(xml("//html_content"))]
    html_content: String,

    #[xee(xml("//sidebar"))]
    sidebar_html: Option<String>,
}

/// Struct for extracting configuration with raw XML
#[derive(Extract)]
struct ConfigWithXML {
    #[xee(xpath("//config/@name"))]
    name: String,

    #[xee(xml("//config/settings"))]
    settings_xml: String,

    #[xee(xml("//config/templates/template"))]
    template_xmls: Vec<String>,
}

/// Struct for extracting document with mixed content
#[derive(Extract)]
struct DocumentWithMixedContent {
    #[xee(xpath("//document/@id"))]
    id: String,

    #[xee(xpath("//document/title"))]
    title: String,

    #[xee(xml("//document/body"))]
    body_xml: String,

    #[xee(xml("//document/footnotes/footnote"))]
    footnote_xmls: Vec<String>,
}

fn main() {
    // Example 1: Basic raw XML extraction
    let basic_xml = r#"
        <article>
            <title>Sample Article</title>
            <content>
                <p>This is a <strong>paragraph</strong> with <em>formatting</em>.</p>
                <ul>
                    <li>Item 1</li>
                    <li>Item 2</li>
                </ul>
            </content>
            <metadata>
                <author>John Doe</author>
                <date>2024-01-15</date>
            </metadata>
            <comments>
                <comment>
                    <author>Alice</author>
                    <text>Great article!</text>
                </comment>
                <comment>
                    <author>Bob</author>
                    <text>Very informative.</text>
                </comment>
            </comments>
        </article>
    "#;

    let extractor = Extractor::default();
    let data: RawXMLData = extractor.extract_from_str(basic_xml).unwrap();

    println!("Basic raw XML extraction:");
    println!("  Title: {}", data.title);
    println!("  Content XML:");
    println!("{}", data.content_xml);
    println!("  Metadata XML: {:?}", data.metadata_xml);
    println!("  Comment XMLs:");
    for (i, comment) in data.comment_xmls.iter().enumerate() {
        println!("    Comment {}: {}", i + 1, comment);
    }
    println!();

    // Example 2: HTML content extraction
    let html_xml = r#"
        <page>
            <title>My Web Page</title>
            <html_content>
                <div class="main">
                    <h1>Welcome</h1>
                    <p>This is the main content with <a href="/link">links</a>.</p>
                    <div class="highlight">
                        <strong>Important notice:</strong> This is highlighted content.
                    </div>
                </div>
            </html_content>
            <sidebar>
                <div class="sidebar">
                    <h3>Navigation</h3>
                    <ul>
                        <li><a href="/home">Home</a></li>
                        <li><a href="/about">About</a></li>
                    </ul>
                </div>
            </sidebar>
        </page>
    "#;

    let html_data: HTMLContent = extractor.extract_from_str(html_xml).unwrap();

    println!("HTML content extraction:");
    println!("  Title: {}", html_data.title);
    println!("  HTML Content:");
    println!("{}", html_data.html_content);
    println!("  Sidebar HTML: {:?}", html_data.sidebar_html);
    println!();

    // Example 3: Configuration with raw XML
    let config_xml = r#"
        <config name="app_settings">
            <settings>
                <database>
                    <host>localhost</host>
                    <port>5432</port>
                    <name>myapp</name>
                </database>
                <api>
                    <base_url>https://api.example.com</base_url>
                    <timeout>30</timeout>
                </api>
            </settings>
            <templates>
                <template name="email">
                    <subject>Welcome to our service</subject>
                    <body>
                        <p>Hello {{name}},</p>
                        <p>Welcome to our platform!</p>
                    </body>
                </template>
                <template name="notification">
                    <subject>New update available</subject>
                    <body>
                        <p>A new version is available.</p>
                    </body>
                </template>
            </templates>
        </config>
    "#;

    let config: ConfigWithXML = extractor.extract_from_str(config_xml).unwrap();

    println!("Configuration with raw XML:");
    println!("  Name: {}", config.name);
    println!("  Settings XML:");
    println!("{}", config.settings_xml);
    println!("  Template XMLs:");
    for (i, template) in config.template_xmls.iter().enumerate() {
        println!("    Template {}: {}", i + 1, template);
    }
    println!();

    // Example 4: Document with mixed content
    let document_xml = r#"
        <document id="DOC001">
            <title>Technical Documentation</title>
            <body>
                <section>
                    <h2>Introduction</h2>
                    <p>This document describes the API.</p>
                    <code>
                        <![CDATA[
                        function example() {
                            return "Hello World";
                        }
                        ]]>
                    </code>
                </section>
                <section>
                    <h2>Usage</h2>
                    <p>Follow these steps:</p>
                    <ol>
                        <li>Step 1</li>
                        <li>Step 2</li>
                    </ol>
                </section>
            </body>
            <footnotes>
                <footnote id="fn1">
                    <text>Additional information about the API.</text>
                    <reference>See section 3.2</reference>
                </footnote>
                <footnote id="fn2">
                    <text>Performance considerations.</text>
                    <reference>See section 4.1</reference>
                </footnote>
            </footnotes>
        </document>
    "#;

    let document: DocumentWithMixedContent = extractor.extract_from_str(document_xml).unwrap();

    println!("Document with mixed content:");
    println!("  ID: {}", document.id);
    println!("  Title: {}", document.title);
    println!("  Body XML:");
    println!("{}", document.body_xml);
    println!("  Footnote XMLs:");
    for (i, footnote) in document.footnote_xmls.iter().enumerate() {
        println!("    Footnote {}: {}", i + 1, footnote);
    }
    println!();
}
