//! Procedural macros for XPath-driven deserialization
//!
//! This module provides the `Extract` derive macro that allows you to
//! deserialize XML documents into Rust structs using XPath expressions.
//!
use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;
use quote::quote;
use syn::{DeriveInput, Attribute, Meta, MetaList, MetaNameValue, NestedMeta, Lit};


#[derive(Debug)]
#[derive(Clone, Copy)]
enum XeeExtractAttributeTag {
    Ns,
    Xpath,
    Context,
    DefaultNs,
    Extract,
    Xml,
}

impl XeeExtractAttributeTag {
    fn from_str(s: &str) -> Option<Self> {
        match s {
            "xpath" => Some(Self::Xpath),
            "ns" => Some(Self::Ns),
            "context" => Some(Self::Context),
            "default_ns" => Some(Self::DefaultNs),
            "extract" => Some(Self::Extract),
            "xml" => Some(Self::Xml),
            _ => None,
        }
    }

    fn allowed_position(&self) -> XeeAttrPosition {
        use XeeAttrPosition::*;
        match self {
            Self::Xpath => Field,
            Self::Ns => Struct,
            Self::Context => Struct,
            Self::DefaultNs => Struct,
            Self::Extract => Field,
            Self::Xml => Field,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum XeeAttrPosition {
    Struct,
    Field,
}

#[derive(Debug)]
struct XeeExtractAttribute {
    /// Type of the attribute (either Xpath or Ns)
    pub attr: XeeExtractAttributeTag,

    /// For `ns(...)` only: the namespace prefix (e.g., `atom` in `atom = "..."`)
    pub attr_key: Option<String>,

    /// Primary string value:
    /// - for `xpath(...)` it's the XPath string (e.g., `"atom:id/text()"`)
    /// - for `ns(...)` it's the namespace URI (e.g., `"http://www.w3.org/2005/Atom"`)
    pub attr_value: String,

    /// Optional trailing string (used as an override variable name or alias)
    pub named_extract: Option<String>,
}

/// Derive macro for XPath-driven deserialization
#[proc_macro_derive(Extract, attributes(xee))]
#[proc_macro_error]
pub fn derive_xee_extract(input: TokenStream) -> TokenStream {
    // Parse the input into a DeriveInput
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    // Call your implementation logic
    match impl_xee_extract(&input) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}
fn impl_xee_extract(input: &DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    // Ensure struct with named fields
    let fields = match &input.data {
        syn::Data::Struct(data) => match &data.fields {
            syn::Fields::Named(named) => &named.named,
            _ => {
                return Err(syn::Error::new_spanned(
                    name,
                    "Extract can only be derived for structs with named fields",
                ))
            }
        },
        _ => {
            return Err(syn::Error::new_spanned(
                name,
                "Extract can only be derived for structs",
            ))
        }
    };

    // Parse struct-level attributes
    let struct_level_attrs = parse_xee_attrs(&input.attrs, XeeAttrPosition::Struct)?;
    let mut static_context_setup = Vec::new();
    let mut context_stmt = quote! {
        let effective_context_item = context_item;
    };

    for attr in &struct_level_attrs {
        match attr.attr {
            XeeExtractAttributeTag::Ns => {
                let key = attr.attr_key.as_ref().ok_or_else(|| {
                    syn::Error::new_spanned(&input.ident, "ns(...) attribute must have a key=value pair like atom = \"uri\"")
                })?;
                let alias = attr.named_extract.as_deref().unwrap_or(key);
                let value = &attr.attr_value;
                static_context_setup.push(quote! {
                    static_context_builder.add_namespace(#alias, #value);
                });
            }

            XeeExtractAttributeTag::DefaultNs => {
                let ns_uri = &attr.attr_value;
                static_context_setup.push(quote! {
                    static_context_builder.default_element_namespace(#ns_uri);
                });
            }

            XeeExtractAttributeTag::Context => {
                let xpath_expr = &attr.attr_value;
                context_stmt = quote! {
                    let effective_context_item = {
                        let context_query = queries.one(#xpath_expr, |documents, item| Ok(item.clone()))?;
                        context_query.execute_build_context(documents, |builder| {
                            builder.context_item(context_item.clone());
                        })?
                    };
                };
            }

            _ => {
                return Err(syn::Error::new_spanned(
                    &input.ident,
                    format!("Unsupported attribute at struct level: {:?}", attr.attr),
                ));
            }
        }
    }

    let context_var = quote! { effective_context_item };

    let mut field_extractions = Vec::new();
    let mut field_names = Vec::new();
    let mut field_values = Vec::new();

    for field in fields {
        let field_ident = field.ident.as_ref().unwrap();
        let xee_attrs = parse_xee_attrs(&field.attrs, XeeAttrPosition::Field)?;

        for xee_attr in xee_attrs {
            let extract_code = generate_extract_for_attr(field_ident, &xee_attr, &context_var, &field.ty)?;
            field_extractions.push(extract_code);
        }

        field_names.push(field_ident);
        field_values.push(quote! { #field_ident });
    }

    let expanded = quote! {
        impl #impl_generics xee_extract::Extract for #name #ty_generics #where_clause {
            fn extract(
                documents: &mut xee_xpath::Documents,
                context_item: &xee_xpath::Item,
            ) -> Result<Self, xee_extract::Error> {
                use xee_xpath::{Queries, Query};

                let mut static_context_builder = xee_xpath::context::StaticContextBuilder::default();

                #(#static_context_setup)*

                let queries = Queries::new(static_context_builder);

                #context_stmt

                #(#field_extractions)*

                Ok(Self {
                    #(#field_names: #field_values,)*
                })
            }
        }
    };

    Ok(expanded)
}

fn generate_extract_for_attr(
    field_ident: &syn::Ident,
    attr: &XeeExtractAttribute,
    context_var: &proc_macro2::TokenStream,
    field_type: &syn::Type,
) -> syn::Result<proc_macro2::TokenStream> {
    use XeeExtractAttributeTag::*;

    match &attr.attr {
        Ns => {
            let key = attr.attr_key.as_ref().ok_or_else(|| {
                syn::Error::new_spanned(
                    field_ident,
                    "ns(...) attribute must have a key=value pair like atom = \"uri\"",
                )
            })?;
            let value = &attr.attr_value;
            let registration = match attr.named_extract.as_deref() {
                Some(alias) => quote! { static_context_builder.add_namespace(#alias, #value); },
                None => quote! { static_context_builder.add_namespace(#key, #value); },
            };
            return Ok(quote! { #registration });
        }

        tag @ (Xpath | Context | DefaultNs | Extract | Xml) => {
            let xpath_expr = &attr.attr_value;
            let extract_id: Option<&str> = attr.named_extract.as_deref();

            if is_vec_u8_type(field_type) || is_option_vec_u8_type(field_type) {
                return generate_vec_u8_query(field_ident, xpath_expr, field_type, context_var);
            }

            let query_method = if is_option_type(field_type) {
                quote! { option }
            } else if is_vec_type(field_type) {
                quote! { many }
            } else {
                quote! { one }
            };

            let inner_type = extract_inner_type(field_type);
            return generate_unified_query(
                xpath_expr,
                inner_type.unwrap_or(field_type),
                *tag,
                context_var,
                query_method,
                field_ident,
                extract_id,
            );
        }
    }
}

fn extract_inner_type(ty: &syn::Type) -> Option<&syn::Type> {
    extract_option_inner_type(ty).or_else(|| extract_vec_inner_type(ty))
}

fn parse_xee_attrs(attrs: &[Attribute], position: XeeAttrPosition) -> syn::Result<Vec<XeeExtractAttribute>> {
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
            let inner_list = match &nested_meta {
                NestedMeta::Meta(Meta::List(list)) => list,
                _ => return Err(syn::Error::new_spanned(&nested_meta, "expected #[xee(tag(...))]")),
            };

            let tag_ident = inner_list.path.get_ident()
                .ok_or_else(|| syn::Error::new_spanned(&inner_list.path, "expected tag ident"))?
                .to_string();

            let tag = XeeExtractAttributeTag::from_str(&tag_ident)
                .ok_or_else(|| syn::Error::new_spanned(inner_list, format!("unknown xee tag: {}", tag_ident)))?;

            if tag.allowed_position() != position {
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
                            NestedMeta::Meta(Meta::NameValue(MetaNameValue { path, lit: Lit::Str(s), .. })) => {
                                let key = path.get_ident()
                                    .ok_or_else(|| syn::Error::new_spanned(path, "expected identifier key"))?
                                    .to_string();
                                attr_key = Some(key);
                                attr_value = Some(s.value());
                            }
                            NestedMeta::Lit(Lit::Str(s)) => {
                                named_extract = Some(s.value());
                            }
                            _ => return Err(syn::Error::new_spanned(item, "unexpected item in ns(...)")),
                        }
                    }

                    let attr = XeeExtractAttribute {
                        attr: tag,
                        attr_key,
                        attr_value: attr_value.ok_or_else(|| syn::Error::new_spanned(&inner_list, "missing ns value"))?,
                        named_extract,
                    };

                    results.push(attr);
                }

                _ => {
                    let mut args = inner_list.nested.iter();

                    let first = match args.next() {
                        Some(NestedMeta::Lit(Lit::Str(s))) => s.value(),
                        Some(other) => return Err(syn::Error::new_spanned(other, "expected string literal")),
                        None => return Err(syn::Error::new_spanned(&inner_list, "missing argument")),
                    };

                    let second = match args.next() {
                        Some(NestedMeta::Lit(Lit::Str(s))) => Some(s.value()),
                        Some(other) => return Err(syn::Error::new_spanned(other, "expected string literal")),
                        None => None,
                    };

                    results.push(XeeExtractAttribute {
                        attr: tag,
                        attr_key: None,
                        attr_value: first,
                        named_extract: second,
                    });
                }
            }
        }
    }

    Ok(results)
}


fn is_option_type(ty: &syn::Type) -> bool {
    if let syn::Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            return segment.ident == "Option";
        }
    }
    false
}

fn is_vec_type(ty: &syn::Type) -> bool {
    if let syn::Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            return segment.ident == "Vec";
        }
    }
    false
}

fn extract_option_inner_type(ty: &syn::Type) -> Option<&syn::Type> {
    if let syn::Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            if segment.ident == "Option" {
                if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                    if let Some(syn::GenericArgument::Type(inner_type)) = args.args.first() {
                        return Some(inner_type);
                    }
                }
            }
        }
    }
    None
}

fn extract_vec_inner_type(ty: &syn::Type) -> Option<&syn::Type> {
    if let syn::Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            if segment.ident == "Vec" {
                if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                    if let Some(syn::GenericArgument::Type(inner_type)) = args.args.first() {
                        return Some(inner_type);
                    }
                }
            }
        }
    }
    None
}

fn is_vec_u8_type(ty: &syn::Type) -> bool {
    if let Some(inner_type) = extract_vec_inner_type(ty) {
        if let syn::Type::Path(type_path) = inner_type {
            if let Some(segment) = type_path.path.segments.last() {
                return segment.ident == "u8";
            }
        }
    }
    false
}

fn is_option_vec_u8_type(ty: &syn::Type) -> bool {
    if let Some(inner_type) = extract_option_inner_type(ty) {
        return is_vec_u8_type(inner_type);
    }
    false
}

fn generate_unified_query(
    xpath_expr: &str,
    field_type: &syn::Type,
    tag: XeeExtractAttributeTag,
    context_var: &proc_macro2::TokenStream,
    query_method: proc_macro2::TokenStream,
    field_name: &syn::Ident,
    extract_id: Option<&str>,
) -> syn::Result<proc_macro2::TokenStream> {
    let field_name_str = field_name.to_string();
    let extract_id_str = extract_id
        .map(|id| format!(" using named extract '{}'", id))
        .unwrap_or_default();

    let combined_msg_prefix = format!(
        "Error extracting value for field '{}'{extract_id_str}: ",
        field_name_str
    );

    let combined_msg_lit = proc_macro2::Literal::string(&combined_msg_prefix);

    let body = match tag {
        XeeExtractAttributeTag::Extract => quote! {
            use xee_extract::Extract;
            <#field_type>::extract(documents, item).map_err(|e| {
                xee_interpreter::error::SpannedError::from(
                    xee_interpreter::error::Error::Application(Box::new(
                        xee_interpreter::error::ApplicationError::new(
                            xot::xmlname::OwnedName::new(
                                "extract-value-error".to_string(),
                                "http://github.com/Paligo/xee/errors".to_string(),
                                "".to_string(),
                            ),
                            format!("{}{}", #combined_msg_lit, e)
                        )
                    ))
                )
            })
        },

        XeeExtractAttributeTag::Xml => quote! {
            match item {
                xee_xpath::Item::Node(node) => {
                    let xml_str = documents.xot().serialize_xml_string(Default::default(), *node)
                        .map_err(|e| xee_interpreter::error::SpannedError::from(
                            xee_interpreter::error::Error::Application(Box::new(
                                xee_interpreter::error::ApplicationError::new(
                                    xot::xmlname::OwnedName::new(
                                        "extract-value-error".to_string(),
                                        "http://github.com/Paligo/xee/errors".to_string(),
                                        "".to_string(),
                                    ),
                                    format!("{}{}", #combined_msg_lit, e)
                                )
                            ))
                        ))?;
                    Ok(xml_str)
                }
                _ => Ok(item.string_value(documents.xot())?)
            }
        },

        _ => quote! {
            use xee_extract::ExtractValue;
            <#field_type>::extract_value(documents, item).map_err(|e| {
                xee_interpreter::error::SpannedError::from(
                    xee_interpreter::error::Error::Application(Box::new(
                        xee_interpreter::error::ApplicationError::new(
                            xot::xmlname::OwnedName::new(
                                "extract-value-error".to_string(),
                                "http://github.com/Paligo/xee/errors".to_string(),
                                "".to_string(),
                            ),
                            format!("{}{}", #combined_msg_lit, e)
                        )
                    ))
                )
            })
        },
    };

    Ok(quote! {
        let #field_name = {
            let query = queries.#query_method(#xpath_expr, |documents, item| {
                #body
            })?;
            query.execute_build_context(documents, |builder| {
                builder.context_item(#context_var.clone());
            })?
        };
    })
}

fn generate_vec_u8_query(
    field_name: &syn::Ident,
    xpath_expr: &str,
    field_type: &syn::Type,
    context_var: &proc_macro2::TokenStream,
) -> syn::Result<proc_macro2::TokenStream> {
    // Check if this is Option<Vec<u8>>
    let is_option = is_option_vec_u8_type(field_type);
    
    let query_method = if is_option { quote! { option } } else { quote! { one } };

    let query_code = quote! {
        let query = queries.#query_method(#xpath_expr, |documents, item| {
            // Special handling for Vec<u8> - check if item is Binary atomic
            match item {
                xee_xpath::Item::Atomic(xee_xpath::Atomic::Binary(binary_type, data)) => {
                    // For binary atomic values, return the data directly
                    Ok(data.as_ref().to_vec())
                }
                _ => {
                    // Just extract the binary value of the string value of the item
                    let string_value = item.string_value(documents.xot())?;
                    Ok(string_value.as_bytes().to_vec())
                }
            }
        })?;
        query.execute_build_context(documents, |builder| {
            builder.context_item(#context_var.clone());
        })?
    };

    let field_name_token = quote! { #field_name };
    Ok(quote! {
        let #field_name_token = {
            #query_code
        };
    })
}


#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_quote;

    fn attr(input: proc_macro2::TokenStream) -> syn::Attribute {
        parse_quote!(#[xee(#input)])
    }

    #[test]
    fn test_xpath_single_arg() {
        let attrs = vec![
            attr(quote!(xpath("foo/bar"))),
        ];

        let parsed = parse_xee_attrs(&attrs).unwrap();
        assert_eq!(parsed.len(), 1);
        let attr = &parsed[0];
        assert!(matches!(attr.attr, XeeExtractAttributeTag::Xpath));
        assert_eq!(attr.attr_value, "foo/bar");
        assert_eq!(attr.named_extract, None);
    }

    #[test]
    fn test_xpath_with_alias() {
        let attrs = vec![
            attr(quote!(xpath("foo/bar", "my_alias"))),
        ];

        let parsed = parse_xee_attrs(&attrs).unwrap();
        assert_eq!(parsed.len(), 1);
        let attr = &parsed[0];
        assert_eq!(attr.attr_value, "foo/bar");
        assert_eq!(attr.named_extract.as_deref(), Some("my_alias"));
    }

    #[test]
    fn test_ns_with_key_value() {
        let attrs = vec![
            attr(quote!(ns(atom = "http://www.w3.org/2005/Atom"))),
        ];

        let parsed = parse_xee_attrs(&attrs).unwrap();
        assert_eq!(parsed.len(), 1);
        let attr = &parsed[0];
        assert!(matches!(attr.attr, XeeExtractAttributeTag::Ns));
        assert_eq!(attr.attr_key.as_deref(), Some("atom"));
        assert_eq!(attr.attr_value, "http://www.w3.org/2005/Atom");
        assert_eq!(attr.named_extract, None);
    }

    #[test]
    fn test_ns_with_key_value_and_alias() {
        let attrs = vec![
            attr(quote!(ns(atom = "http://www.w3.org/2005/Atom", "ns_alias"))),
        ];

        let parsed = parse_xee_attrs(&attrs).unwrap();
        assert_eq!(parsed.len(), 1);
        let attr = &parsed[0];
        assert_eq!(attr.attr_key.as_deref(), Some("atom"));
        assert_eq!(attr.attr_value, "http://www.w3.org/2005/Atom");
        assert_eq!(attr.named_extract.as_deref(), Some("ns_alias"));
    }

    #[test]
    fn test_multiple_attributes() {
        let attrs = vec![
            attr(quote!(xpath("id"))),
            attr(quote!(ns(atom = "http://atom", "alias"))),
            attr(quote!(context("ctx", "ctx_alias"))),
        ];

        let parsed = parse_xee_attrs(&attrs).unwrap();
        assert_eq!(parsed.len(), 3);

        assert!(matches!(parsed[0].attr, XeeExtractAttributeTag::Xpath));
        assert!(matches!(parsed[1].attr, XeeExtractAttributeTag::Ns));
        assert!(matches!(parsed[2].attr, XeeExtractAttributeTag::Context));
    }

    #[test]
    fn test_invalid_tag() {
        let attrs = vec![
            attr(quote!(nonsense("abc"))),
        ];

        let result = parse_xee_attrs(&attrs);
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_ns_missing_value() {
        let attrs = vec![
            attr(quote!(ns(atom = 123))),
        ];

        let result = parse_xee_attrs(&attrs);
        assert!(result.is_err());
    }
}