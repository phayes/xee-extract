use xee_extract::{Extract, Extractor};

fn default_title() -> String {
    "generated".to_string()
}

fn default_opt() -> Option<String> {
    Some("fallback".to_string())
}

fn field_default() -> String {
    "field_default".to_string()
}

#[derive(Extract, Debug, PartialEq)]
struct FieldDefaults {
    #[xee(xpath("//id/text()"))]
    id: String,

    #[xee(xpath("//missing/text()"))]
    #[xee(default)]
    name: String,

    #[xee(default("default_title"))]
    title: String,
}

#[test]
fn test_field_defaults() {
    let xml = "<root><id>1</id></root>";
    let res: FieldDefaults = Extractor::default().extract_from_str(xml).unwrap();
    assert_eq!(res.id, "1");
    assert_eq!(res.name, String::default());
    assert_eq!(res.title, "generated");
}

#[derive(Extract, Debug, PartialEq)]
struct OptionDefault {
    #[xee(default("default_opt"))]
    value: Option<String>,
}

#[test]
fn test_option_default() {
    let xml = "<root/>";
    let res: OptionDefault = Extractor::default().extract_from_str(xml).unwrap();
    assert_eq!(res.value, Some("fallback".to_string()));
}

#[derive(Extract, Debug, PartialEq)]
#[xee(default)]
struct StructDefault {
    #[xee(xpath("//id/text()"))]
    id: String,
    // no attribute, will come from Default
    other: i32,
}

impl Default for StructDefault {
    fn default() -> Self {
        Self {
            id: String::new(),
            other: 42,
        }
    }
}

#[test]
fn test_struct_default() {
    let xml = "<root><id>7</id></root>";
    let res: StructDefault = Extractor::default().extract_from_str(xml).unwrap();
    assert_eq!(res.id, "7");
    assert_eq!(res.other, 42);
}

// Test named extracts with defaults
#[derive(Extract, Debug, PartialEq)]
#[xee(ns(nlm = "https://id.nlm.nih.gov/datmm/", "nlm"))]
struct NamedExtractWithDefaults {
    #[xee(xpath("//id/text()"))]
    #[xee(xpath("//nlm:id/text()", "nlm"))]
    id: String,

    #[xee(xpath("//missing/text()"))]
    #[xee(xpath("//nlm:missing/text()", "nlm"))]
    #[xee(default)]
    name: String,

    #[xee(default("default_title"))]
    title: String,

    #[xee(xpath("//author/text()"))]
    #[xee(xpath("//nlm:author/text()", "nlm"))]
    #[xee(default("default_opt"))]
    author: Option<String>,
}

impl Default for NamedExtractWithDefaults {
    fn default() -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            title: String::new(),
            author: None,
        }
    }
}

#[test]
fn test_named_extract_with_defaults() {
    // Test default extraction
    let xml = "<root><id>1</id><author>Alice</author></root>";
    let res: NamedExtractWithDefaults = Extractor::default().extract_from_str(xml).unwrap();
    assert_eq!(res.id, "1");
    assert_eq!(res.name, String::default());
    assert_eq!(res.title, "generated");
    assert_eq!(res.author, Some("Alice".to_string()));

    // Test named extraction
    let nlm_xml = "<root xmlns:nlm=\"https://id.nlm.nih.gov/datmm/\"><nlm:id>2</nlm:id><nlm:author>Bob</nlm:author></root>";
    let res: NamedExtractWithDefaults = Extractor::named("nlm").extract_from_str(nlm_xml).unwrap();
    assert_eq!(res.id, "2");
    assert_eq!(res.name, String::default());
    assert_eq!(res.title, "generated");
    assert_eq!(res.author, Some("Bob".to_string()));
}

#[test]
fn test_named_extract_with_missing_values() {
    // Test default extraction with missing values
    let xml = "<root><id>1</id></root>";
    let res: NamedExtractWithDefaults = Extractor::default().extract_from_str(xml).unwrap();
    assert_eq!(res.id, "1");
    assert_eq!(res.name, String::default());
    assert_eq!(res.title, "generated");
    assert_eq!(res.author, Some("fallback".to_string()));

    // Test named extraction with missing values
    let nlm_xml = "<root xmlns:nlm=\"https://id.nlm.nih.gov/datmm/\"><nlm:id>2</nlm:id></root>";
    let res: NamedExtractWithDefaults = Extractor::named("nlm").extract_from_str(nlm_xml).unwrap();
    assert_eq!(res.id, "2");
    assert_eq!(res.name, String::default());
    assert_eq!(res.title, "generated");
    assert_eq!(res.author, Some("fallback".to_string()));
}

// Test struct-level defaults with named extracts
#[derive(Extract, Debug, PartialEq)]
#[xee(default)]
#[xee(ns(nlm = "https://id.nlm.nih.gov/datmm/", "nlm"))]
struct StructDefaultWithNamedExtract {
    #[xee(xpath("//id/text()"))]
    #[xee(xpath("//nlm:id/text()", "nlm"))]
    id: String,

    #[xee(xpath("//title/text()"))]
    #[xee(xpath("//nlm:title/text()", "nlm"))]
    title: String,

    // Fields without xee attributes should get struct default
    count: i32,
    active: bool,
}

impl Default for StructDefaultWithNamedExtract {
    fn default() -> Self {
        Self {
            id: String::new(),
            title: String::new(),
            count: 100,
            active: true,
        }
    }
}

#[test]
fn test_struct_default_with_named_extract() {
    // Test default extraction
    let xml = "<root><id>1</id><title>Default Title</title></root>";
    let res: StructDefaultWithNamedExtract = Extractor::default().extract_from_str(xml).unwrap();
    assert_eq!(res.id, "1");
    assert_eq!(res.title, "Default Title");
    assert_eq!(res.count, 100);
    assert_eq!(res.active, true);

    // Test named extraction
    let nlm_xml = "<root xmlns:nlm=\"https://id.nlm.nih.gov/datmm/\"><nlm:id>2</nlm:id><nlm:title>NLM Title</nlm:title></root>";
    let res: StructDefaultWithNamedExtract =
        Extractor::named("nlm").extract_from_str(nlm_xml).unwrap();
    assert_eq!(res.id, "2");
    assert_eq!(res.title, "NLM Title");
    assert_eq!(res.count, 100);
    assert_eq!(res.active, true);
}

// Test mixed field and struct defaults with named extracts
#[derive(Extract, Debug, PartialEq)]
#[xee(default)]
#[xee(ns(nlm = "https://id.nlm.nih.gov/datmm/", "nlm"))]
struct MixedDefaultsWithNamedExtract {
    #[xee(xpath("//id/text()"))]
    #[xee(xpath("//nlm:id/text()", "nlm"))]
    id: String,

    #[xee(xpath("//missing/text()"))]
    #[xee(xpath("//nlm:missing/text()", "nlm"))]
    #[xee(default("default_title"))]
    name: String,

    // This field will use struct default
    count: i32,

    #[xee(default("default_opt"))]
    metadata: Option<String>,
}

impl Default for MixedDefaultsWithNamedExtract {
    fn default() -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            count: 42,
            metadata: None,
        }
    }
}

#[test]
fn test_mixed_defaults_with_named_extract() {
    // Test default extraction
    let xml = "<root><id>1</id></root>";
    let res: MixedDefaultsWithNamedExtract = Extractor::default().extract_from_str(xml).unwrap();
    assert_eq!(res.id, "1");
    assert_eq!(res.name, "generated");
    assert_eq!(res.count, 42);
    assert_eq!(res.metadata, Some("fallback".to_string()));

    // Test named extraction
    let nlm_xml = "<root xmlns:nlm=\"https://id.nlm.nih.gov/datmm/\"><nlm:id>2</nlm:id></root>";
    let res: MixedDefaultsWithNamedExtract =
        Extractor::named("nlm").extract_from_str(nlm_xml).unwrap();
    assert_eq!(res.id, "2");
    assert_eq!(res.name, "generated");
    assert_eq!(res.count, 42);
    assert_eq!(res.metadata, Some("fallback".to_string()));
}

// Test multiple named extracts with different defaults
#[derive(Extract, Debug, PartialEq)]
#[xee(ns(nlm = "https://id.nlm.nih.gov/datmm/", "nlm"))]
#[xee(ns(atom = "http://www.w3.org/2005/Atom", "atom"))]
struct MultipleNamedExtracts {
    #[xee(xpath("//id/text()"))]
    #[xee(xpath("//nlm:id/text()", "nlm"))]
    #[xee(xpath("//atom:id/text()", "atom"))]
    id: String,

    #[xee(default("default_title"))]
    title: String,

    #[xee(xpath("//author/text()"))]
    #[xee(xpath("//nlm:author/text()", "nlm"))]
    #[xee(xpath("//atom:author/text()", "atom"))]
    #[xee(default("default_opt"))]
    author: Option<String>,
}

impl Default for MultipleNamedExtracts {
    fn default() -> Self {
        Self {
            id: String::new(),
            title: String::new(),
            author: None,
        }
    }
}

#[test]
fn test_multiple_named_extracts() {
    // Test default extraction
    let xml = "<root><id>1</id><author>Alice</author></root>";
    let res: MultipleNamedExtracts = Extractor::default().extract_from_str(xml).unwrap();
    assert_eq!(res.id, "1");
    assert_eq!(res.title, "generated");
    assert_eq!(res.author, Some("Alice".to_string()));

    // Test NLM named extraction
    let nlm_xml = "<root xmlns:nlm=\"https://id.nlm.nih.gov/datmm/\"><nlm:id>2</nlm:id><nlm:author>Bob</nlm:author></root>";
    let res: MultipleNamedExtracts = Extractor::named("nlm").extract_from_str(nlm_xml).unwrap();
    assert_eq!(res.id, "2");
    assert_eq!(res.title, "generated");
    assert_eq!(res.author, Some("Bob".to_string()));

    // Test Atom named extraction
    let atom_xml = "<root xmlns:atom=\"http://www.w3.org/2005/Atom\"><atom:id>3</atom:id><atom:author>Carol</atom:author></root>";
    let res: MultipleNamedExtracts = Extractor::named("atom").extract_from_str(atom_xml).unwrap();
    assert_eq!(res.id, "3");
    assert_eq!(res.title, "generated");
    assert_eq!(res.author, Some("Carol".to_string()));
}

#[test]
fn test_multiple_named_extracts_with_missing_values() {
    // Test default extraction with missing values
    let xml = "<root><id>1</id></root>";
    let res: MultipleNamedExtracts = Extractor::default().extract_from_str(xml).unwrap();
    assert_eq!(res.id, "1");
    assert_eq!(res.title, "generated");
    assert_eq!(res.author, Some("fallback".to_string()));

    // Test NLM named extraction with missing values
    let nlm_xml = "<root xmlns:nlm=\"https://id.nlm.nih.gov/datmm/\"><nlm:id>2</nlm:id></root>";
    let res: MultipleNamedExtracts = Extractor::named("nlm").extract_from_str(nlm_xml).unwrap();
    assert_eq!(res.id, "2");
    assert_eq!(res.title, "generated");
    assert_eq!(res.author, Some("fallback".to_string()));

    // Test Atom named extraction with missing values
    let atom_xml = "<root xmlns:atom=\"http://www.w3.org/2005/Atom\"><atom:id>3</atom:id></root>";
    let res: MultipleNamedExtracts = Extractor::named("atom").extract_from_str(atom_xml).unwrap();
    assert_eq!(res.id, "3");
    assert_eq!(res.title, "generated");
    assert_eq!(res.author, Some("fallback".to_string()));
}

// Test that field-level defaults override struct-level defaults
#[derive(Extract, Debug, PartialEq)]
#[xee(default)]
struct FieldOverridesStruct {
    #[xee(default("field_default"))]
    value: String,
}

impl Default for FieldOverridesStruct {
    fn default() -> Self {
        Self {
            value: "struct_default".to_string(),
        }
    }
}

#[test]
fn test_field_default_overrides_struct_default() {
    let xml = "<root/>";
    let res: FieldOverridesStruct = Extractor::default().extract_from_str(xml).unwrap();
    assert_eq!(res.value, "field_default"); // Should use field default, not struct default
}
