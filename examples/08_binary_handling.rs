//! Example 8: Binary Handling
//!
//! This example demonstrates how to handle binary data in xee-extract,
//! including hex and base64 encoded binary data.

use xee_extract::{Extract, Extractor};

/// Struct for handling base64 encoded binary data
#[derive(Extract)]
#[xee(ns(xs = "http://www.w3.org/2001/XMLSchema"))]
struct Base64Data {
    #[xee(xpath("xs:base64Binary(//base64-data)"))]
    data: Vec<u8>,

    #[xee(xpath("xs:base64Binary(//optional-base64)"))]
    optional_data: Option<Vec<u8>>,

    #[xee(xpath("xs:base64Binary(/root/@data)"))]
    attr_data: Vec<u8>,
}

/// Struct for handling hex encoded binary data
#[derive(Extract)]
#[xee(ns(xs = "http://www.w3.org/2001/XMLSchema"))]
struct HexData {
    #[xee(xpath("xs:hexBinary(//hex-data)"))]
    data: Vec<u8>,

    #[xee(xpath("xs:hexBinary(//optional-hex)"))]
    optional_data: Option<Vec<u8>>,

    #[xee(xpath("xs:hexBinary(//hex-node)"))]
    node_data: Vec<u8>,
}

/// Struct for mixed binary data types
#[derive(Extract)]
#[xee(ns(xs = "http://www.w3.org/2001/XMLSchema"))]
struct MixedBinaryData {
    #[xee(xpath("xs:base64Binary(//base64-field)"))]
    base64_field: Vec<u8>,

    #[xee(xpath("xs:hexBinary(//hex-field)"))]
    hex_field: Vec<u8>,

    #[xee(xpath("xs:base64Binary(//optional-binary)"))]
    optional_binary: Option<Vec<u8>>,
}

/// Struct for binary data with metadata
#[derive(Extract)]
#[xee(ns(xs = "http://www.w3.org/2001/XMLSchema"))]
struct BinaryWithMetadata {
    #[xee(xpath("//binary/@name"))]
    name: String,

    #[xee(xpath("//binary/@type"))]
    binary_type: String,

    #[xee(xpath("xs:base64Binary(//binary/data)"))]
    data: Vec<u8>,

    #[xee(xpath("//binary/size"))]
    size: u32,

    #[xee(xpath("//binary/checksum"))]
    checksum: Option<String>,
}

fn main() {
    // Example 1: Base64 encoded binary data
    let base64_xml = r#"
        <root data="VGVzdCBkYXRh">
            <base64-data>SGVsbG8gV29ybGQ=</base64-data>
            <optional-base64>U29tZSBkYXRh</optional-base64>
        </root>
    "#;

    let extractor = Extractor::new();
    let data: Base64Data = extractor.extract_from_str(base64_xml).unwrap();

    println!("Base64 encoded binary data:");
    println!("  Data: {:?}", data.data);
    println!("  Data as string: {}", String::from_utf8_lossy(&data.data));
    println!("  Optional data: {:?}", data.optional_data);
    if let Some(ref opt_data) = data.optional_data {
        println!(
            "  Optional data as string: {}",
            String::from_utf8_lossy(opt_data)
        );
    }
    println!("  Attribute data: {:?}", data.attr_data);
    println!(
        "  Attribute as string: {}",
        String::from_utf8_lossy(&data.attr_data)
    );
    println!();

    // Example 2: Hex encoded binary data
    let hex_xml = r#"
        <root>
            <hex-data>48656C6C6F20576F726C64</hex-data>
            <optional-hex>536F6D652064617461</optional-hex>
            <hex-node>546573742064617461</hex-node>
        </root>
    "#;

    let data: HexData = extractor.extract_from_str(hex_xml).unwrap();

    println!("Hex encoded binary data:");
    println!("  Data: {:?}", data.data);
    println!("  Data as string: {}", String::from_utf8_lossy(&data.data));
    println!("  Optional data: {:?}", data.optional_data);
    if let Some(ref opt_data) = data.optional_data {
        println!(
            "  Optional data as string: {}",
            String::from_utf8_lossy(opt_data)
        );
    }
    println!("  Node data: {:?}", data.node_data);
    println!(
        "  Node data as string: {}",
        String::from_utf8_lossy(&data.node_data)
    );
    println!();

    // Example 3: Mixed binary data types
    let mixed_xml = r#"
        <root>
            <base64-field>SGVsbG8gV29ybGQ=</base64-field>
            <hex-field>48656C6C6F20576F726C64</hex-field>
            <optional-binary>U29tZSBkYXRh</optional-binary>
        </root>
    "#;

    let data: MixedBinaryData = extractor.extract_from_str(mixed_xml).unwrap();

    println!("Mixed binary data types:");
    println!("  Base64 field: {:?}", data.base64_field);
    println!(
        "  Base64 as string: {}",
        String::from_utf8_lossy(&data.base64_field)
    );
    println!("  Hex field: {:?}", data.hex_field);
    println!(
        "  Hex as string: {}",
        String::from_utf8_lossy(&data.hex_field)
    );
    println!("  Optional binary: {:?}", data.optional_binary);
    if let Some(ref opt_binary) = data.optional_binary {
        println!(
            "  Optional binary as string: {}",
            String::from_utf8_lossy(opt_binary)
        );
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

    let data: BinaryWithMetadata = extractor.extract_from_str(metadata_xml).unwrap();

    println!("Binary data with metadata:");
    println!("  Name: {}", data.name);
    println!("  Type: {}", data.binary_type);
    println!("  Data: {:?}", data.data);
    println!("  Data as string: {}", String::from_utf8_lossy(&data.data));
    println!("  Size: {}", data.size);
    println!("  Checksum: {:?}", data.checksum);
    println!();
}
