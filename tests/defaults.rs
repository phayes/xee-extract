use xee_extract::{Extract, Extractor};

fn default_title() -> String {
    "generated".to_string()
}

fn default_opt() -> Option<String> {
    Some("fallback".to_string())
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
