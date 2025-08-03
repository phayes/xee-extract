#![doc = include_str!("../README.md")]

use std::str::FromStr;
use xee_xpath::{Atomic, Documents, Item, Sequence};
use xot::xmlname::OwnedName;

// Re-export the macro
pub use xee_extract_macros::Extract;

// Error types module
mod error;

// Re-export error types
pub use error::{Error, ExtractError, FieldExtractionError};

pub trait Extract: Sized {
    /// Extract from an XML node (for recursive extraction)
    fn extract_from_node(
        documents: &mut Documents,
        item: &Item,
        extract_id: Option<&str>,
        variables: &ahash::AHashMap<OwnedName, Sequence>,
    ) -> Result<Self, Error> {
        match item {
            Item::Node(_) => Self::extract(documents, item, extract_id, variables),
            _ => {
                return Result::Err(Error::InvalidXPath(
                    "extract targets non-node items".to_string(),
                ));
            }
        }
    }

    /// Extract from a context item using XPath expressions relative to that item
    fn extract(
        documents: &mut Documents,
        context_item: &Item,
        extract_id: Option<&str>,
        variables: &ahash::AHashMap<OwnedName, Sequence>,
    ) -> Result<Self, Error>;
}

/// Trait for deserializing a type from an XPath item
pub trait ExtractValue: Sized {
    /// Deserialize a value from an XPath item
    fn extract_value(documents: &mut Documents, item: &Item) -> Result<Self, Error>;
}

/// Default ExtractValue impl that punts to FromStr
impl<T> ExtractValue for T
where
    T: FromStr,
    T::Err: std::fmt::Display,
{
    fn extract_value(documents: &mut Documents, item: &Item) -> Result<Self, Error> {
        let s = item.string_value(documents.xot())?;
        s.parse::<T>()
            .map_err(|e| Error::DeserializationError(e.to_string()))
    }
}

/// Extractor for XML documents using XPath expressions
pub struct Extractor {
    variables: std::collections::HashMap<String, Sequence>,
    extract_name: Option<String>,
}

impl Default for Extractor {
    fn default() -> Self {
        Self {
            variables: std::collections::HashMap::new(),
            extract_name: None,
        }
    }
}

impl Extractor {
    /// Create a new extractor with a named extraction
    /// 
    /// Sometimes a single struct needs to support multiple XML formats.  Each
    /// `#[xee(...)]` attribute can take an optional second string argument that
    /// associates it with a named extraction.  When using
    /// `Extractor::named("nlm")`, only the attributes tagged with that name are
    /// applied; attributes without a name form the default extraction used by
    ///  `Extractor::default()`.
    ///
    /// ```rust
    /// use xee_extract::{Extractor, Extract};
    ///
    /// #[derive(Extract)]
    /// #[xee(ns(atom = "http://www.w3.org/2005/Atom"))]                // default
    /// #[xee(ns(nlm = "https://id.nlm.nih.gov/datmm/", "nlm"))]        // named
    /// struct Entry {
    ///     #[xee(xpath("//atom:id/text()"))]                          // default
    ///     #[xee(xpath("//nlm:id/text()", "nlm"))]                    // named
    ///     id: String,
    ///
    ///     #[xee(xpath("//atom:title/text()"))]                       // default
    ///     #[xee(xpath("//nlm:title/text()", "nlm"))]                 // named
    ///     title: String,
    /// }
    ///
    /// let atom_xml = r#"<entry xmlns:atom="http://www.w3.org/2005/Atom"><atom:id>123</atom:id><atom:title>Atom Title</atom:title></entry>"#;
    /// let nlm_xml = r#"<entry xmlns:nlm="https://id.nlm.nih.gov/datmm/"><nlm:id>456</nlm:id><nlm:title>NLM Title</nlm:title></entry>"#;
    ///
    /// // Parse Atom
    /// let atom: Entry = Extractor::default().extract_from_str(atom_xml).unwrap();
    ///
    /// // Parse NLM using the named extraction
    /// let nlm: Entry = Extractor::named("nlm").extract_from_str(nlm_xml).unwrap();
    /// ```
    pub fn named(name: &str) -> Self {
        Self {
            variables: std::collections::HashMap::new(),
            extract_name: Some(name.to_string()),
        }
    }

    /// Bind a sequence to a variable, sequence can be anything that can be converted to an xee_xpath::Sequence.
    ///  
    /// This method is safe to call multiple times with the same name (the previous value will be replaced).
    pub fn bind_sequence<S: Into<String>, V: Into<Sequence>>(mut self, name: S, val: V) -> Self {
        self.variables.insert(name.into(), val.into());
        self
    }

    /// Bind an item to a variable, item can be anything that can be converted to an xee_xpath::Item.
    /// For example a reference to an existing xee_xpath::Item or node.
    ///
    /// This method is safe to call multiple times with the same name (the previous value will be replaced).
    pub fn bind_item<S: Into<String>, V: Into<Item>>(self, name: S, val: V) -> Self {
        let item: Item = val.into();
        self.bind_sequence(name, item)
    }

    /// Bind a value to a variable, value can be anything that can be converted to an Atomic.
    /// This includes String, &str, f64, f32, i64, i32, u64, u32, bool, and other types that can be converted to an xee_xpath::Atomic.
    ///
    /// This method is safe to call multiple times with the same name (the previous value will be replaced).
    ///
    /// Example:
    /// ```rust
    /// use xee_extract::Extractor;
    /// let extractor = Extractor::default().bind_value("name", "John Doe").bind_value("is_student", true);
    /// ```
    pub fn bind_value<S: Into<String>, V: Into<Atomic>>(self, name: S, val: V) -> Self {
        let atomic: Atomic = val.into();
        self.bind_item(name, atomic)
    }

    /// Extract a single struct from an XML string
    pub fn extract_from_str<T>(&self, xml: &str) -> Result<T, ExtractError>
    where
        T: Extract,
    {
        let mut documents = xee_xpath::Documents::new();
        let doc = documents
            .add_string_without_uri(xml)
            .map_err(|e| ExtractError::new(Error::DocumentsError(e), &xml))?;

        use xee_xpath::Itemable;
        let item = doc
            .to_item(&mut documents)
            .map_err(|e| ExtractError::new(Error::SpannedError(e), &xml))?;

        // Bind the variables.
        let mut variables: ahash::AHashMap<OwnedName, Sequence> =
            ahash::AHashMap::with_capacity(self.variables.len());
        for (name, sequence) in self.variables.iter() {
            variables.insert(OwnedName::name(name), sequence.clone());
        }

        // Use the trait's deserialize method
        let res = T::extract(
            &mut documents,
            &item,
            self.extract_name.as_deref(),
            &variables,
        );

        match res {
            Ok(value) => Ok(value),
            Err(error) => Err(ExtractError::new(error, &xml))?,
        }
    }

    /// Extract a single struct from a documents store
    pub fn extract_from_docs<T>(
        &self,
        documents: &mut xee_xpath::Documents,
        root_doc: &xee_xpath::DocumentHandle,
    ) -> Result<T, ExtractError>
    where
        T: Extract,
    {
        use xee_xpath::Itemable;
        let item = root_doc
            .to_item(documents)
            .map_err(|e| ExtractError::no_span(Error::SpannedError(e)))?;

        // Bind the variables.
        let mut variables: ahash::AHashMap<OwnedName, Sequence> =
            ahash::AHashMap::with_capacity(self.variables.len());
        for (name, sequence) in self.variables.iter() {
            variables.insert(OwnedName::name(name), sequence.clone());
        }

        // Use the trait's deserialize method
        let res = T::extract(documents, &item, self.extract_name.as_deref(), &variables);

        match res {
            Ok(value) => Ok(value),
            Err(error) => Err(ExtractError::no_span(error))?,
        }
    }
}
