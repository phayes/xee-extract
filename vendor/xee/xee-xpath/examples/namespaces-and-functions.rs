use xee_xpath::context::StaticContextBuilder;
use xee_xpath::{DocumentHandle, Documents, Queries, Query};

/// A higher-level wrapper around Xee's XPath functionality
struct Document<'a> {
    documents: Documents,
    queries: Queries<'a>,
    doc_handle: DocumentHandle,
}

impl<'a> Document<'a> {
    /// Create a new document with XML content and namespace mappings
    fn new(xml: &str, namespaces: &[(&'a str, &'a str)]) -> Self {
        let mut documents = Documents::new();
        let doc_handle = documents.add_string_without_uri(xml).unwrap();

        let mut builder = StaticContextBuilder::default();
        for (prefix, uri) in namespaces {
            builder.add_namespace(prefix, uri);
        }

        let queries = Queries::new(builder);
        Self {
            documents,
            queries,
            doc_handle,
        }
    }

    /// Execute an XPath query and return the result as a string
    fn query(&mut self, xpath: &str) -> String {
        self.queries
            .sequence(xpath)
            .and_then(|it| it.execute(&mut self.documents, self.doc_handle))
            .unwrap()
            .string_value(self.documents.xot())
            .unwrap()
    }
}

fn main() {
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<grocery-list xmlns:item="http://example.com/item"
              xmlns:store="http://example.com/store">
    <item:description>Weekly grocery shopping list</item:description>
    <item:category name="produce" store:section="A1">
        <item:product id="p1" organic="true" quantity="6">
            <item:name>Banana</item:name>
            <item:price currency="USD">0.25</item:price>
            <item:unit>each</item:unit>
            <store:availability>In stock</store:availability>
        </item:product>
        <item:product id="p2" organic="false" quantity="1">
            <item:name>Lettuce</item:name>
            <item:price currency="USD">1.99</item:price>
            <item:unit>head</item:unit>
            <store:availability>Limited</store:availability>
        </item:product>
    </item:category>

    <item:category name="dairy" store:section="B3">
        <item:product id="d1" organic="true" quantity="1">
            <item:name>Milk</item:name>
            <item:price currency="USD">3.49</item:price>
            <item:unit>gallon</item:unit>
            <store:availability>In stock</store:availability>
        </item:product>
        <item:product id="d2" organic="false" quantity="2">
            <item:name>Cheese</item:name>
            <item:price currency="USD">2.99</item:price>
            <item:unit>package</item:unit>
            <store:availability>In stock</store:availability>
        </item:product>
    </item:category>

    <item:category name="pantry" store:section="C2">
        <item:product id="pn1" organic="false" quantity="1">
            <item:name>Pasta</item:name>
            <item:price currency="USD">1.49</item:price>
            <item:unit>box</item:unit>
            <store:availability>In stock</store:availability>
        </item:product>
        <item:product id="pn2" organic="false" quantity="2">
            <item:name>Canned Tomatoes</item:name>
            <item:price currency="USD">0.99</item:price>
            <item:unit>can</item:unit>
            <store:availability>Out of stock</store:availability>
        </item:product>
    </item:category>
</grocery-list>"#;

    let namespaces = [
        ("item", "http://example.com/item"),
        ("store", "http://example.com/store"),
    ];
    let mut document = Document::new(xml, &namespaces);

    // Get the list description
    let description = document.query("/grocery-list/item:description");
    println!("List description: {description}");

    // Get all products
    let products =
        document.query("string-join(/grocery-list/item:category/item:product/item:name, ', ')");
    println!("Products: {products}");

    // Find organic products
    let organic_products =
        document.query("string-join(//item:product[@organic='true']/item:name, ', ')");
    println!("Organic products: {organic_products}");

    // Count products in a category
    let count = document.query("count(//item:category[@name='produce']/item:product)");
    println!("Number of produce items: {count}");

    // Calculate total cost of available items only
    let total_cost = document.query(
        "sum(for $p in //item:product[store:availability='In stock'] return number($p/item:price) * number($p/@quantity))"
    );
    println!("Total cost of in-stock items: {total_cost}");

    // Calculate average price per product
    let avg_price =
        document.query("round-half-to-even(avg(//item:product/item:price/number()), 2)");
    println!("Average price per product: ${avg_price}");
}
