#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct XmlName {
    pub namespace: String,
    pub local: String,
}
