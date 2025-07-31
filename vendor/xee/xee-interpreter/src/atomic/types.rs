use xee_schema_type::Xs;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IntegerType {
    Integer,
    NonPositiveInteger,
    NegativeInteger,
    NonNegativeInteger,
    PositiveInteger,
    Long,
    Int,
    Short,
    Byte,
    UnsignedLong,
    UnsignedInt,
    UnsignedShort,
    UnsignedByte,
}

impl IntegerType {
    pub(crate) fn schema_type(&self) -> Xs {
        match self {
            IntegerType::Integer => Xs::Integer,
            IntegerType::Long => Xs::Long,
            IntegerType::Int => Xs::Int,
            IntegerType::Short => Xs::Short,
            IntegerType::Byte => Xs::Byte,
            IntegerType::UnsignedLong => Xs::UnsignedLong,
            IntegerType::UnsignedInt => Xs::UnsignedInt,
            IntegerType::UnsignedShort => Xs::UnsignedShort,
            IntegerType::UnsignedByte => Xs::UnsignedByte,
            IntegerType::NonPositiveInteger => Xs::NonPositiveInteger,
            IntegerType::NegativeInteger => Xs::NegativeInteger,
            IntegerType::NonNegativeInteger => Xs::NonNegativeInteger,
            IntegerType::PositiveInteger => Xs::PositiveInteger,
        }
    }
}

/// The types of string supported as atomic values.
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StringType {
    /// xs:string
    String,
    /// xs:normalizedString
    NormalizedString,
    /// xs:token
    Token,
    /// xs:language
    Language,
    /// xs:NMTOKEN
    NMTOKEN,
    /// xs:Name
    Name,
    /// xs:NCName
    NCName,
    /// xs:ID
    ID,
    /// xs:IDREF
    IDREF,
    /// xs:ENTITY
    ENTITY,
    // the qt3 tests make the assumption AnyURI is a type of string
    /// xs:anyURI
    AnyURI,
}

impl StringType {
    pub(crate) fn schema_type(&self) -> Xs {
        match self {
            StringType::String => Xs::String,
            StringType::NormalizedString => Xs::NormalizedString,
            StringType::Token => Xs::Token,
            StringType::Language => Xs::Language,
            StringType::NMTOKEN => Xs::NMTOKEN,
            StringType::Name => Xs::Name,
            StringType::NCName => Xs::NCName,
            StringType::ID => Xs::ID,
            StringType::IDREF => Xs::IDREF,
            StringType::ENTITY => Xs::ENTITY,
            StringType::AnyURI => Xs::AnyURI,
        }
    }
}

/// The types of binary supported as atomic values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BinaryType {
    /// xs:base64Binary
    Base64,
    /// xs:hexBinary
    Hex,
}

impl BinaryType {
    pub(crate) fn schema_type(&self) -> Xs {
        match self {
            BinaryType::Base64 => Xs::Base64Binary,
            BinaryType::Hex => Xs::HexBinary,
        }
    }
}
