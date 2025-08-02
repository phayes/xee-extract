//! Example 8: Binary Handling
//! 
//! This example demonstrates how to handle binary data in xee-extract,
//! including hex and base64 encoded binary data.

use xee_extract::{Extractor, Extract};

/// Struct for handling base64 encoded binary data
#[derive(Extract, Debug, PartialEq)]
#[xee(ns(xs = "http://www.w3.org/2001/XMLSchema"))]
struct Base64Data {
    #[xee(xpath("string(//base64-data/text()) cast as xs:base64Binary"))]
    data: Vec<u8>,

    #[xee(xpath("string(//optional-base64/text()) cast as xs:base64Binary"))]
    optional_data: Option<Vec<u8>>,

    #[xee(xpath("string(//base64-node/text()) cast as xs:base64Binary"))]
    node_data: Vec<u8>,
}

/// Struct for handling hex encoded binary data
#[derive(Extract, Debug, PartialEq)]
#[xee(ns(xs = "http://www.w3.org/2001/XMLSchema"))]
struct HexData {
    #[xee(xpath("string(//hex-data/text()) cast as xs:hexBinary"))]
    data: Vec<u8>,

    #[xee(xpath("string(//optional-hex/text()) cast as xs:hexBinary"))]
    optional_data: Option<Vec<u8>>,

    #[xee(xpath("string(//hex-node/text()) cast as xs:hexBinary"))]
    node_data: Vec<u8>,
}

/// Struct for mixed binary data types
#[derive(Extract, Debug, PartialEq)]
#[xee(ns(xs = "http://www.w3.org/2001/XMLSchema"))]
struct MixedBinaryData {
    #[xee(xpath("string(//base64-field/text()) cast as xs:base64Binary"))]
    base64_field: Vec<u8>,

    #[xee(xpath("string(//hex-field/text()) cast as xs:hexBinary"))]
    hex_field: Vec<u8>,

    #[xee(xpath("string(//optional-binary/text()) cast as xs:base64Binary"))]
    optional_binary: Option<Vec<u8>>,
}

/// Struct for binary data with metadata
#[derive(Extract, Debug, PartialEq)]
#[xee(ns(xs = "http://www.w3.org/2001/XMLSchema"))]
struct BinaryWithMetadata {
    #[xee(xpath("//binary/@name"))]
    name: String,

    #[xee(xpath("//binary/@type"))]
    binary_type: String,

    #[xee(xpath("string(//binary/data/text()) cast as xs:base64Binary"))]
    data: Vec<u8>,

    #[xee(xpath("//binary/size/text()"))]
    size: u32,

    #[xee(xpath("//binary/checksum/text()"))]
    checksum: Option<String>,
}

fn main() {
    // Example 1: Base64 encoded binary data
    let base64_xml = r#"
        <root>
            <base64-data>SGVsbG8gV29ybGQ=</base64-data>
            <optional-base64>U29tZSBkYXRh</optional-base64>
            <base64-node>VGVzdCBkYXRh</base64-node>
        </root>
    "#;

    let extractor = Extractor::new();
    let data: Base64Data = extractor.extract_one(base64_xml).unwrap();

    println!("Base64 encoded binary data:");
    println!("  Data: {:?}", data.data);
    println!("  Data as string: {}", String::from_utf8_lossy(&data.data));
    println!("  Optional data: {:?}", data.optional_data);
    if let Some(ref opt_data) = data.optional_data {
        println!("  Optional data as string: {}", String::from_utf8_lossy(opt_data));
    }
    println!("  Node data: {:?}", data.node_data);
    println!("  Node data as string: {}", String::from_utf8_lossy(&data.node_data));
    println!();

    // Example 2: Hex encoded binary data
    let hex_xml = r#"
        <root>
            <hex-data>48656C6C6F20576F726C64</hex-data>
            <optional-hex>536F6D652064617461</optional-hex>
            <hex-node>546573742064617461</hex-node>
        </root>
    "#;

    let data: HexData = extractor.extract_one(hex_xml).unwrap();

    println!("Hex encoded binary data:");
    println!("  Data: {:?}", data.data);
    println!("  Data as string: {}", String::from_utf8_lossy(&data.data));
    println!("  Optional data: {:?}", data.optional_data);
    if let Some(ref opt_data) = data.optional_data {
        println!("  Optional data as string: {}", String::from_utf8_lossy(opt_data));
    }
    println!("  Node data: {:?}", data.node_data);
    println!("  Node data as string: {}", String::from_utf8_lossy(&data.node_data));
    println!();

    // Example 3: Mixed binary data types
    let mixed_xml = r#"
        <root>
            <base64-field>SGVsbG8gV29ybGQ=</base64-field>
            <hex-field>48656C6C6F20576F726C64</hex-field>
            <optional-binary>U29tZSBkYXRh</optional-binary>
        </root>
    "#;

    let data: MixedBinaryData = extractor.extract_one(mixed_xml).unwrap();

    println!("Mixed binary data types:");
    println!("  Base64 field: {:?}", data.base64_field);
    println!("  Base64 as string: {}", String::from_utf8_lossy(&data.base64_field));
    println!("  Hex field: {:?}", data.hex_field);
    println!("  Hex as string: {}", String::from_utf8_lossy(&data.hex_field));
    println!("  Optional binary: {:?}", data.optional_binary);
    if let Some(ref opt_binary) = data.optional_binary {
        println!("  Optional binary as string: {}", String::from_utf8_lossy(opt_binary));
    }
    println!();

    // Example 4: Binary data with metadata
    let metadata_xml = r#"
        <root>
            <binary name="config_file" type="base64">
                <data>U2FtcGxlIGNvbmZpZyBkYXRh</data>
                <size>18</size>
                <checksum>a1b2c3d4</checksum>
            </binary>
        </root>
    "#;

    let data: BinaryWithMetadata = extractor.extract_one(metadata_xml).unwrap();

    println!("Binary data with metadata:");
    println!("  Name: {}", data.name);
    println!("  Type: {}", data.binary_type);
    println!("  Data: {:?}", data.data);
    println!("  Data as string: {}", String::from_utf8_lossy(&data.data));
    println!("  Size: {}", data.size);
    println!("  Checksum: {:?}", data.checksum);
    println!();

    // Example 5: Error handling for invalid binary data
    let invalid_xml = r#"
        <root>
            <base64-data>Invalid base64 data!</base64-data>
        </root>
    "#;

    let result = extractor.extract_one::<Base64Data>(invalid_xml);
    
    println!("Error handling for invalid binary data:");
    match result {
        Ok(data) => println!("  Unexpected success: {:?}", data),
        Err(e) => println!("  Expected error: {}", e),
    }

    // Example 6: Empty binary data
    let empty_xml = r#"
        <root>
            <base64-data></base64-data>
            <hex-data></hex-data>
        </root>
    "#;

    let data: Base64Data = extractor.extract_one(empty_xml).unwrap();

    println!("Empty binary data:");
    println!("  Data: {:?}", data.data);
    println!("  Data length: {}", data.data.len());
    println!();

    // Example 7: Large binary data demonstration
    let large_xml = r#"
        <root>
            <base64-data>VGhpcyBpcyBhIGxvbmdlciBzdHJpbmcgdG8gZGVtb25zdHJhdGUgYmluYXJ5IGRhdGEgaGFuZGxpbmc=</base64-data>
        </root>
    "#;

    let data: Base64Data = extractor.extract_one(large_xml).unwrap();

    println!("Large binary data:");
    println!("  Data length: {}", data.data.len());
    println!("  Data as string: {}", String::from_utf8_lossy(&data.data));
    println!();

    // Example 8: Binary data with special characters
    let special_xml = r#"
        <root>
            <base64-data>8J+RjSDwn5GNIPCfkY4=</base64-data>
        </root>
    "#;

    let data: Base64Data = extractor.extract_one(special_xml).unwrap();

    println!("Binary data with special characters:");
    println!("  Data: {:?}", data.data);
    println!("  Data as string: {}", String::from_utf8_lossy(&data.data));
    println!("  Data as hex: {:02x?}", data.data);
} 