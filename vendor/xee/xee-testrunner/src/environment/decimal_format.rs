type Name = xot::xmlname::OwnedName;

/// Only is used by XPath tests, not XSLT
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct DecimalFormat {
    pub(crate) name: Name,
    pub(crate) decimal_separator: Option<char>,
    pub(crate) grouping_separator: Option<char>,
    pub(crate) zero_digit: Option<char>,
    pub(crate) digit: Option<char>,
    pub(crate) minus_sign: Option<char>,
    pub(crate) percent: Option<char>,
    pub(crate) per_mille: Option<char>,
    pub(crate) pattern_separator: Option<char>,
    pub(crate) exponent_separator: Option<char>,
    pub(crate) infinity: Option<String>,
    pub(crate) nan: Option<String>,
}
