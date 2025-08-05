//! Attribute parsing and validation
//!
//! This module provides the `XeeExtractAttribute` struct, which is used to store the parsed attribute
//! information.

use syn::{Attribute, Lit, Meta, MetaList, MetaNameValue, NestedMeta};

// All attribute tags that are supported by xee-extract
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum XeeExtractAttributeTag {
    Ns,
    Xpath,
    Context,
    DefaultNs,
    Extract,
    Xml,
    Default,
}

impl XeeExtractAttributeTag {
    pub(crate) fn from_str(s: &str) -> Option<Self> {
        match s {
            "xpath" => Some(Self::Xpath),
            "ns" => Some(Self::Ns),
            "context" => Some(Self::Context),
            "default_ns" => Some(Self::DefaultNs),
            "extract" => Some(Self::Extract),
            "xml" => Some(Self::Xml),
            "default" => Some(Self::Default),
            _ => None,
        }
    }

    pub(crate) fn allowed_position(&self) -> &[XeeAttrPosition] {
        use XeeAttrPosition::*;
        match self {
            Self::Xpath => &[Field],
            Self::Ns => &[Struct],
            Self::Context => &[Struct],
            Self::DefaultNs => &[Struct],
            Self::Extract => &[Field],
            Self::Xml => &[Field],
            Self::Default => &[Struct, Field],
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(crate) enum XeeAttrPosition {
    Struct,
    Field,
}

// Parsed attribute from the derive macro input
#[derive(Debug)]
pub(crate) struct XeeExtractAttribute {
    /// Type of the attribute (either Xpath or Ns)
    pub attr: XeeExtractAttributeTag,

    /// For `ns(...)` only: the namespace prefix (e.g., `atom` in `atom = "..."`)
    pub attr_key: Option<String>,

    /// Primary string value:
    /// - for `xpath(...)` it's the XPath string (e.g., `"atom:id/text()"`)
    /// - for `ns(...)` it's the namespace URI (e.g., `"http://www.w3.org/2005/Atom"`)
    /// - for `default` we could have either:
    ///     - #[xee(default)], which indicates that Default::default() should be called. attr_value is an empty string.
    ///     - #[xee(default("my_function"))], which indicates that my_function() should be called. The function must be callable as fn() -> T.
    pub attr_value: String,

    /// Optional trailing string (used as an override variable name or alias)
    pub named_extract: Option<String>,
}

impl XeeExtractAttribute {
    pub(crate) fn parse_many(
        attrs: &[Attribute],
        position: XeeAttrPosition,
    ) -> syn::Result<Vec<Self>> {
        let mut results = Vec::new();

        for attr in attrs {
            if !attr.path.is_ident("xee") {
                continue;
            }

            let meta = attr.parse_meta()?;
            let Meta::List(MetaList { nested, .. }) = meta else {
                return Err(syn::Error::new_spanned(attr, "expected #[xee(...)]"));
            };

            for nested_meta in nested {
                let parsed_attr = Self::parse_one(&nested_meta, position)?;
                results.push(parsed_attr);
            }
        }

        Ok(results)
    }

    fn parse_one(nested_meta: &NestedMeta, position: XeeAttrPosition) -> syn::Result<Self> {
        match nested_meta {
            NestedMeta::Meta(Meta::List(inner_list)) => {
                let tag_ident = inner_list
                    .path
                    .get_ident()
                    .ok_or_else(|| {
                        syn::Error::new_spanned(
                            &inner_list.path,
                            "expected xee tag like xee(xpath(...)) etc.",
                        )
                    })?
                    .to_string();

                let tag = XeeExtractAttributeTag::from_str(&tag_ident).ok_or_else(|| {
                    syn::Error::new_spanned(inner_list, format!("unknown xee tag: {}", tag_ident))
                })?;

                if !tag.allowed_position().contains(&position) {
                    return Err(syn::Error::new_spanned(
                        inner_list,
                        format!("attribute {:?} not allowed on {:?}", tag, position),
                    ));
                }

                match tag {
                    XeeExtractAttributeTag::Ns => {
                        let mut attr_key = None;
                        let mut attr_value = None;
                        let mut named_extract = None;

                        for item in &inner_list.nested {
                            match item {
                                NestedMeta::Meta(Meta::NameValue(MetaNameValue {
                                    path,
                                    lit: Lit::Str(s),
                                    ..
                                })) => {
                                    let key = path
                                        .get_ident()
                                        .ok_or_else(|| {
                                            syn::Error::new_spanned(path, "expected identifier key")
                                        })?
                                        .to_string();
                                    attr_key = Some(key);
                                    attr_value = Some(s.value());
                                }
                                NestedMeta::Lit(Lit::Str(s)) => {
                                    named_extract = Some(s.value());
                                }
                                _ => {
                                    return Err(syn::Error::new_spanned(
                                        item,
                                        "unexpected item in ns(...)",
                                    ))
                                }
                            }
                        }

                        let attr = XeeExtractAttribute {
                            attr: tag,
                            attr_key,
                            attr_value: attr_value.ok_or_else(|| {
                                syn::Error::new_spanned(&inner_list, "missing ns value")
                            })?,
                            named_extract,
                        };

                        Ok(attr)
                    }

                    XeeExtractAttributeTag::Default => {
                        let mut args = inner_list.nested.iter();
                        let value = match args.next() {
                            Some(NestedMeta::Lit(Lit::Str(s))) => s.value(),
                            Some(other) => {
                                return Err(syn::Error::new_spanned(
                                    other,
                                    "expected string literal",
                                ))
                            }
                            None => String::new(),
                        };

                        // Check for additional arguments that would become named_extract
                        if let Some(additional_arg) = args.next() {
                            return Err(syn::Error::new_spanned(
                                additional_arg,
                                "named_extract is not supported for default attributes",
                            ));
                        }

                        Ok(XeeExtractAttribute {
                            attr: tag,
                            attr_key: None,
                            attr_value: value,
                            named_extract: None,
                        })
                    }

                    _ => {
                        let mut args = inner_list.nested.iter();

                        let first = match args.next() {
                            Some(NestedMeta::Lit(Lit::Str(s))) => s.value(),
                            Some(other) => {
                                return Err(syn::Error::new_spanned(
                                    other,
                                    "expected string literal",
                                ))
                            }
                            None => {
                                return Err(syn::Error::new_spanned(
                                    &inner_list,
                                    "missing argument",
                                ))
                            }
                        };

                        let second = match args.next() {
                            Some(NestedMeta::Lit(Lit::Str(s))) => Some(s.value()),
                            Some(other) => {
                                return Err(syn::Error::new_spanned(
                                    other,
                                    "expected string literal",
                                ))
                            }
                            None => None,
                        };

                        Ok(XeeExtractAttribute {
                            attr: tag,
                            attr_key: None,
                            attr_value: first,
                            named_extract: second,
                        })
                    }
                }
            }
            NestedMeta::Meta(Meta::Path(path)) => {
                let tag_ident = path
                    .get_ident()
                    .ok_or_else(|| syn::Error::new_spanned(path, "expected tag ident"))?
                    .to_string();
                let tag = XeeExtractAttributeTag::from_str(&tag_ident).ok_or_else(|| {
                    syn::Error::new_spanned(path, format!("unknown xee tag: {}", tag_ident))
                })?;

                // Only Default is allowed as a single-argument attribute
                if tag != XeeExtractAttributeTag::Default {
                    return Err(syn::Error::new_spanned(path, "expected #[xee(tag(...))]"));
                }

                if !tag.allowed_position().contains(&position) {
                    return Err(syn::Error::new_spanned(
                        path,
                        format!("attribute {:?} not allowed on {:?}", tag, position),
                    ));
                }

                Ok(XeeExtractAttribute {
                    attr: tag,
                    attr_key: None,
                    attr_value: String::new(),
                    named_extract: None,
                })
            }
            _ => Err(syn::Error::new_spanned(
                nested_meta,
                "expected #[xee(tag(...))]",
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quote::quote;
    use syn::parse_quote;

    fn attr(input: proc_macro2::TokenStream) -> syn::Attribute {
        parse_quote!(#[xee(#input)])
    }

    #[test]
    fn test_xpath_single_arg() {
        let attrs = vec![attr(quote!(xpath("foo/bar")))];

        let parsed = XeeExtractAttribute::parse_many(&attrs, XeeAttrPosition::Field).unwrap();
        assert_eq!(parsed.len(), 1);
        let attr = &parsed[0];
        assert!(matches!(attr.attr, XeeExtractAttributeTag::Xpath));
        assert_eq!(attr.attr_value, "foo/bar");
        assert_eq!(attr.named_extract, None);
    }

    #[test]
    fn test_xpath_with_alias() {
        let attrs = vec![attr(quote!(xpath("foo/bar", "my_alias")))];

        let parsed = XeeExtractAttribute::parse_many(&attrs, XeeAttrPosition::Field).unwrap();
        assert_eq!(parsed.len(), 1);
        let attr = &parsed[0];
        assert_eq!(attr.attr_value, "foo/bar");
        assert_eq!(attr.named_extract.as_deref(), Some("my_alias"));
    }

    #[test]
    fn test_ns_with_key_value() {
        let attrs = vec![attr(quote!(ns(atom = "http://www.w3.org/2005/Atom")))];

        let parsed = XeeExtractAttribute::parse_many(&attrs, XeeAttrPosition::Struct).unwrap();
        assert_eq!(parsed.len(), 1);
        let attr = &parsed[0];
        assert!(matches!(attr.attr, XeeExtractAttributeTag::Ns));
        assert_eq!(attr.attr_key.as_deref(), Some("atom"));
        assert_eq!(attr.attr_value, "http://www.w3.org/2005/Atom");
        assert_eq!(attr.named_extract, None);
    }

    #[test]
    fn test_ns_with_key_value_and_alias() {
        let attrs = vec![attr(quote!(ns(
            atom = "http://www.w3.org/2005/Atom",
            "ns_alias"
        )))];

        let parsed = XeeExtractAttribute::parse_many(&attrs, XeeAttrPosition::Struct).unwrap();
        assert_eq!(parsed.len(), 1);
        let attr = &parsed[0];
        assert_eq!(attr.attr_key.as_deref(), Some("atom"));
        assert_eq!(attr.attr_value, "http://www.w3.org/2005/Atom");
        assert_eq!(attr.named_extract.as_deref(), Some("ns_alias"));
    }

    #[test]
    fn test_multiple_attributes() {
        let attrs = vec![
            attr(quote!(ns(atom = "http://atom", "alias"))),
            attr(quote!(context("ctx", "ctx_alias"))),
        ];

        let parsed = XeeExtractAttribute::parse_many(&attrs, XeeAttrPosition::Struct).unwrap();
        assert_eq!(parsed.len(), 2);

        assert!(matches!(parsed[0].attr, XeeExtractAttributeTag::Ns));
        assert!(matches!(parsed[1].attr, XeeExtractAttributeTag::Context));
    }

    #[test]
    fn test_invalid_tag() {
        let attrs = vec![attr(quote!(nonsense("abc")))];

        let result = XeeExtractAttribute::parse_many(&attrs, XeeAttrPosition::Struct);
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_ns_missing_value() {
        let attrs = vec![attr(quote!(ns(atom = 123)))];

        let result = XeeExtractAttribute::parse_many(&attrs, XeeAttrPosition::Struct);
        assert!(result.is_err());
    }

    #[test]
    fn test_default_attr() {
        let attrs = vec![attr(quote!(default))];

        let parsed = XeeExtractAttribute::parse_many(&attrs, XeeAttrPosition::Struct).unwrap();
        assert_eq!(parsed.len(), 1);
        let attr = &parsed[0];
        assert!(matches!(attr.attr, XeeExtractAttributeTag::Default));
        assert_eq!(attr.attr_value, "");
        assert_eq!(attr.named_extract, None);
    }

    #[test]
    fn test_default_attr_with_function() {
        let attrs = vec![attr(quote!(default("my_function")))];

        let parsed = XeeExtractAttribute::parse_many(&attrs, XeeAttrPosition::Struct).unwrap();
        assert_eq!(parsed.len(), 1);
        let attr = &parsed[0];
        assert!(matches!(attr.attr, XeeExtractAttributeTag::Default));
        assert_eq!(attr.attr_value, "my_function");
        assert_eq!(attr.named_extract, None);
    }

    #[test]
    fn test_default_attr_with_named_extract_error() {
        let attrs = vec![attr(quote!(default("my_function", "named_extract")))];

        let result = XeeExtractAttribute::parse_many(&attrs, XeeAttrPosition::Struct);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("named_extract is not supported for default attributes"));
    }
}
