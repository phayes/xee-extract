use xee_extract::{Extractor, Extract};

#[derive(Extract, Debug, PartialEq)]
#[xee(ns(xs = "http://www.w3.org/2001/XMLSchema"))]
struct BinaryData {
    #[xee(xpath("string(//binary-data/text()) cast as xs:base64Binary"))]
    base64_data: Vec<u8>,

    #[xee(xpath("string(//hex-data/text()) cast as xs:hexBinary"))]
    hex_data: Vec<u8>,

    #[xee(xpath("string(//binary-node/text()) cast as xs:base64Binary"))]
    binary_node: Vec<u8>,

    #[xee(xpath("string(//binary-node/text()) cast as xs:base64Binary"))]
    binary_xml: Vec<u8>,
}

#[derive(Extract, Debug, PartialEq)]
#[xee(ns(xs = "http://www.w3.org/2001/XMLSchema"))]
struct BinaryWithOption {
    #[xee(xpath("string(//optional-binary/text()) cast as xs:base64Binary"))]
    optional_binary: Option<Vec<u8>>,

    #[xee(xpath("string(//required-binary/text()) cast as xs:hexBinary"))]
    required_binary: Vec<u8>,
}

#[test]
fn test_binary_extraction_base64() {
    let xml = r#"
        <root>
            <binary-data>SGVsbG8gV29ybGQ=</binary-data>
            <hex-data>48656C6C6F20576F726C64</hex-data>
            <binary-node>SGVsbG8gV29ybGQ=</binary-node>
        </root>
    "#;

    let extractor = Extractor::new();
    let result: BinaryData = extractor.extract_from_str(xml).unwrap();

    // "Hello World" in base64
    let expected_base64 = b"Hello World".to_vec();
    // "Hello World" in hex
    let expected_hex = b"Hello World".to_vec();

    assert_eq!(result.base64_data, expected_base64);
    assert_eq!(result.hex_data, expected_hex);
    assert_eq!(result.binary_node, expected_base64);
    assert_eq!(result.binary_xml, expected_base64);
}

#[test]
fn test_binary_extraction_with_option() {
    let xml = r#"
        <root>
            <optional-binary>SGVsbG8=</optional-binary>
            <required-binary>48656C6C6F</required-binary>
        </root>
    "#;

    let extractor = Extractor::new();
    let result: BinaryWithOption = extractor.extract_from_str(xml).unwrap();

    // "Hello" in base64
    let expected_optional = b"Hello".to_vec();
    // "Hello" in hex
    let expected_required = b"Hello".to_vec();

    assert_eq!(result.optional_binary, Some(expected_optional));
    assert_eq!(result.required_binary, expected_required);
}

#[test]
fn test_binary_extraction_missing_optional() {
    let xml = r#"
        <root>
            <required-binary>48656C6C6F</required-binary>
        </root>
    "#;

    let extractor = Extractor::new();
    let result: BinaryWithOption = extractor.extract_from_str(xml).unwrap();

    // "Hello" in hex
    let expected_required = b"Hello".to_vec();

    // When the optional field is missing, it returns Some([]) instead of None
    // This is because the XPath finds an empty string and converts it to an empty Vec<u8>
    assert_eq!(result.optional_binary, Some(Vec::<u8>::new()));
    assert_eq!(result.required_binary, expected_required);
}

#[test]
fn test_binary_extraction_invalid_data() {
    let xml = r#"
        <root>
            <binary-data>Invalid base64 data!</binary-data>
        </root>
    "#;

    let extractor = Extractor::new();
    let result = extractor.extract_from_str::<BinaryData>(xml);

    // Should fail because the data is not valid base64
    assert!(result.is_err());
}

#[test]
fn test_binary_extraction_empty_data() {
    let xml = r#"
        <root>
            <binary-data></binary-data>
            <hex-data></hex-data>
        </root>
    "#;

    let extractor = Extractor::new();
    let result: BinaryData = extractor.extract_from_str(xml).unwrap();

    // Empty data should result in empty Vec<u8>
    assert_eq!(result.base64_data, Vec::<u8>::new());
    assert_eq!(result.hex_data, Vec::<u8>::new());
} 