use xee_extract::{Extract};

fn default_title() -> String {
    "generated".to_string()
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
