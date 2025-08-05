//! Procedural macros for XPath-driven deserialization
//!
//! This module provides the `Extract` derive macro that allows you to
//! deserialize XML documents into Rust structs using XPath expressions.
//!
use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;
use quote::quote;
use syn::{
    parse_str, Attribute, DeriveInput, Lit, Meta, MetaList, MetaNameValue, NestedMeta, Path,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum XeeExtractAttributeTag {
    Ns,
    Xpath,
    Context,
    DefaultNs,
    Extract,
    Xml,
    Default,
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
            "default" => Some(Self::Default),
            _ => None,
        }
    }

    fn allowed_position(&self) -> &[XeeAttrPosition] {
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
    /// - for `default` we could have either:
    ///     - #[xee(default)], which indicates that Default::default() should be called. attr_value is an empty string.
    ///     - #[xee(default("my_function"))], which indicates that my_function() should be called. The function must be callable as fn() -> T.
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
    use std::collections::HashMap;

    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

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
    let struct_default_attrs: Vec<_> = struct_level_attrs
        .iter()
        .filter(|a| a.attr == XeeExtractAttributeTag::Default)
        .collect();
    if struct_default_attrs.len() > 1 {
        return Err(syn::Error::new_spanned(
            &input.ident,
            "multiple #[xee(default)] attributes on struct",
        ));
    }
    let struct_default_expr = if let Some(attr) = struct_default_attrs.first() {
        Some(default_expr_from_attr(attr)?)
    } else {
        None
    };

    // Collect static context setup instructions per extract_id
    let mut static_context_setup: HashMap<Option<String>, Vec<proc_macro2::TokenStream>> =
        HashMap::new();
    let mut context_stmt_by_extract: HashMap<Option<String>, proc_macro2::TokenStream> =
        HashMap::new();

    for attr in &struct_level_attrs {
        if attr.attr == XeeExtractAttributeTag::Default {
            continue;
        }

        let key = attr.named_extract.clone();
        let entry = static_context_setup.entry(key.clone()).or_default();

        match attr.attr {
            XeeExtractAttributeTag::Ns => {
                let ns_prefix = attr.attr_key.as_ref().ok_or_else(|| {
                    syn::Error::new_spanned(
                        &input.ident,
                        "ns(...) must have key=value like atom = \"uri\"",
                    )
                })?;
                let ns_uri = &attr.attr_value;
                entry.push(quote! {
                    static_context_builder.add_namespace(#ns_prefix, #ns_uri);
                });
            }

            XeeExtractAttributeTag::DefaultNs => {
                let ns_uri = &attr.attr_value;
                entry.push(quote! {
                    static_context_builder.default_element_namespace(#ns_uri);
                });
            }

            XeeExtractAttributeTag::Context => {
                let xpath_expr = &attr.attr_value;
                context_stmt_by_extract.insert(
                    key,
                    quote! {
                        let effective_context_item = {
                            let context_query = queries.one(#xpath_expr, |documents, item| Ok(item.clone()))?;
                            context_query.execute_build_context(documents, |builder| {
                                builder.context_item(context_item.clone());
                            })?
                        };
                    },
                );
            }

            _ => {
                return Err(syn::Error::new_spanned(
                    &input.ident,
                    format!("Unsupported attribute at struct level: {:?}", attr.attr),
                ))
            }
        }
    }

    // Group field extractions by extract_id
    let mut group_extractions: HashMap<Option<String>, Vec<proc_macro2::TokenStream>> =
        HashMap::new();
    let mut group_fields: HashMap<Option<String>, Vec<(&syn::Ident, proc_macro2::TokenStream)>> =
        HashMap::new();

    let has_struct_default = struct_default_expr.is_some();

    for field in fields {
        let field_ident = field.ident.as_ref().unwrap();
        let xee_attrs = parse_xee_attrs(&field.attrs, XeeAttrPosition::Field)?;

        let mut default_attr = None;
        let mut other_attrs = Vec::new();
        for attr in xee_attrs {
            if attr.attr == XeeExtractAttributeTag::Default {
                default_attr = Some(attr);
            } else {
                other_attrs.push(attr);
            }
        }

        let default_expr = if let Some(attr) = default_attr.as_ref() {
            Some(default_expr_from_attr(attr)?)
        } else {
            None
        };

        if other_attrs.is_empty() {
            if let Some(expr) = default_expr {
                group_extractions
                    .entry(None)
                    .or_default()
                    .push(quote! { let #field_ident = { #expr }; });
                group_fields
                    .entry(None)
                    .or_default()
                    .push((field_ident, quote! { #field_ident }));
            } else if has_struct_default {
                // Field will be provided by struct default
            } else {
                return Err(syn::Error::new_spanned(
                    field_ident,
                    format!("field `{}` has no xee attribute or default", field_ident),
                ));
            }
        } else {
            for xee_attr in other_attrs {
                let group_key = xee_attr.named_extract.clone();
                let extract_code = generate_extract_for_attr(
                    field_ident,
                    &xee_attr,
                    &quote! { effective_context_item },
                    &field.ty,
                    default_expr.clone(),
                )?;

                group_extractions
                    .entry(group_key.clone())
                    .or_default()
                    .push(extract_code);
                group_fields
                    .entry(group_key)
                    .or_default()
                    .push((field_ident, quote! { #field_ident }));
            }
        }
    }

    // Build match arms for extract_id
    let mut match_arms = Vec::new();

    for (key, stmts) in &group_extractions {
        let field_inits = &group_fields[key];
        let field_names: Vec<_> = field_inits.iter().map(|(ident, _)| ident).collect();
        let field_values: Vec<_> = field_inits.iter().map(|(_, val)| val).collect();

        let static_setup = static_context_setup.get(key).into_iter().flatten();
        let context_stmt = context_stmt_by_extract
            .get(key)
            .cloned()
            .unwrap_or_else(|| {
                quote! {
                    let effective_context_item = context_item;
                }
            });

        let key_arm = match key {
            Some(s) => quote! { Some(#s) },
            None => quote! { None },
        };

        let struct_default_tokens = struct_default_expr.as_ref().map(|expr| quote! { ..#expr });

        match_arms.push(quote! {
            #key_arm => {
                let mut static_context_builder = xee_xpath::context::StaticContextBuilder::default();

                if variables.len() > 0 {
                    static_context_builder.variable_names(variables.keys().cloned());
                }

                #(#static_setup)*
                let queries = Queries::new(static_context_builder);
                #context_stmt
                #(#stmts)*
                Ok(Self {
                    #(#field_names: #field_values,)*
                    #struct_default_tokens
                })
            }
        });
    }

    match_arms.push(quote! {
        Some(other) => Err(xee_extract::Error::UnknownExtractId(other.to_string()))
    });

    let expanded = quote! {
        impl #impl_generics xee_extract::Extract for #name #ty_generics #where_clause {
            fn extract(
                documents: &mut xee_xpath::Documents,
                context_item: &xee_xpath::Item,
                extract_id: Option<&str>,
                variables: &ahash::AHashMap<xot::xmlname::OwnedName, xee_xpath::Sequence>,
            ) -> Result<Self, xee_extract::Error> {
                use xee_xpath::{Queries, Query};

                match extract_id {
                    #(#match_arms),*
                }
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
    default_expr: Option<proc_macro2::TokenStream>,
) -> syn::Result<proc_macro2::TokenStream> {
    use XeeExtractAttributeTag::*;

    match &attr.attr {
        tag @ (Xpath | Extract | Xml) => {
            let xpath_expr = &attr.attr_value;
            let extract_id: Option<&str> = attr.named_extract.as_deref();

            if is_vec_u8_type(field_type) || is_option_vec_u8_type(field_type) {
                return generate_vec_u8_query(
                    field_ident,
                    xpath_expr,
                    context_var,
                    field_type,
                    default_expr,
                );
            }

            return generate_unified_query(
                xpath_expr,
                field_type,
                *tag,
                context_var,
                field_ident,
                extract_id,
                default_expr,
            );
        }

        _ => {
            panic!("Unsupported attribute at field level: {:?}. This should never happen and is a bug in xee-extract", attr.attr);
        }
    }
}

fn parse_xee_attrs(
    attrs: &[Attribute],
    position: XeeAttrPosition,
) -> syn::Result<Vec<XeeExtractAttribute>> {
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
            match &nested_meta {
                NestedMeta::Meta(Meta::List(inner_list)) => {
                    let tag_ident = inner_list
                        .path
                        .get_ident()
                        .ok_or_else(|| {
                            syn::Error::new_spanned(&inner_list.path, "expected tag ident")
                        })?
                        .to_string();

                    let tag = XeeExtractAttributeTag::from_str(&tag_ident).ok_or_else(|| {
                        syn::Error::new_spanned(
                            inner_list,
                            format!("unknown xee tag: {}", tag_ident),
                        )
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
                                                syn::Error::new_spanned(
                                                    path,
                                                    "expected identifier key",
                                                )
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

                            results.push(attr);
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

                            // default(...) does not support named extracts
                            results.push(XeeExtractAttribute {
                                attr: tag,
                                attr_key: None,
                                attr_value: value,
                                named_extract: None,
                            });
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

                            results.push(XeeExtractAttribute {
                                attr: tag,
                                attr_key: None,
                                attr_value: first,
                                named_extract: second,
                            });
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

                    if tag != XeeExtractAttributeTag::Default {
                        return Err(syn::Error::new_spanned(path, "expected #[xee(tag(...))]"));
                    }

                    if !tag.allowed_position().contains(&position) {
                        return Err(syn::Error::new_spanned(
                            path,
                            format!("attribute {:?} not allowed on {:?}", tag, position),
                        ));
                    }

                    results.push(XeeExtractAttribute {
                        attr: tag,
                        attr_key: None,
                        attr_value: String::new(),
                        named_extract: None,
                    });
                }
                _ => {
                    return Err(syn::Error::new_spanned(
                        &nested_meta,
                        "expected #[xee(tag(...))]",
                    ));
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

fn is_f32_type(ty: &syn::Type) -> bool {
    matches!(ty, syn::Type::Path(type_path) if type_path.qself.is_none() && type_path.path.is_ident("f32"))
}

fn is_f64_type(ty: &syn::Type) -> bool {
    matches!(ty, syn::Type::Path(type_path) if type_path.qself.is_none() && type_path.path.is_ident("f64"))
}

fn is_bool_type(ty: &syn::Type) -> bool {
    matches!(ty, syn::Type::Path(type_path) if type_path.qself.is_none() && type_path.path.is_ident("bool"))
}

fn default_expr_from_attr(attr: &XeeExtractAttribute) -> syn::Result<proc_macro2::TokenStream> {
    if attr.attr_value.is_empty() {
        Ok(quote! { Default::default() })
    } else {
        let path: Path = parse_str(&attr.attr_value)?;
        Ok(quote! { #path() })
    }
}

fn generate_unified_query(
    xpath_expr: &str,
    field_type: &syn::Type,
    tag: XeeExtractAttributeTag,
    context_var: &proc_macro2::TokenStream,
    field_name: &syn::Ident,
    extract_id: Option<&str>,
    default_expr: Option<proc_macro2::TokenStream>,
) -> syn::Result<proc_macro2::TokenStream> {
    let field_name_str = field_name.to_string();
    let xpath_expr_lit = proc_macro2::Literal::string(xpath_expr);
    let extract_id_lit = extract_id.map(proc_macro2::Literal::string);

    let extract_id_match = match extract_id_lit {
        Some(lit) => quote! { Some(#lit) },
        None => quote! { None },
    };

    let (field_type, outer_field_type) = if is_option_type(field_type) {
        (extract_option_inner_type(field_type).unwrap(), field_type)
    } else if is_vec_type(field_type) {
        (extract_vec_inner_type(field_type).unwrap(), field_type)
    } else {
        (field_type, field_type)
    };

    let query_method = if is_vec_type(outer_field_type)
        || (is_option_type(outer_field_type) && is_vec_type(field_type))
    {
        quote! { many }
    } else {
        quote! { option }
    };

    let extractor = match tag {
        XeeExtractAttributeTag::Extract => {
            let extract_id_expr = extract_id
                .map(|id| quote! { Some(#id) })
                .unwrap_or_else(|| quote! { None });
            quote! {
                use xee_extract::Extract;
                <#field_type>::extract(documents, item, #extract_id_expr, variables).map_err(|e| {
                    xee_interpreter::error::SpannedError::from(
                        xee_interpreter::error::Error::Application(Box::new(
                            xee_interpreter::error::ApplicationError::new(
                                xot::xmlname::OwnedName::new(
                                    "extract-value-error".to_string(),
                                    "http://github.com/Paligo/xee/errors".to_string(),
                                    "".to_string(),
                                ),
                                format!("{}", e)
                            )
                        ))
                    )
                })
            }
        }

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
                                    format!("{}", e)
                                )
                            ))
                        ))?;
                    Ok(xml_str)
                }
                _ => Ok(item.string_value(documents.xot())?)
            }
        },

        XeeExtractAttributeTag::Xpath => {
            let hot_path = if is_f32_type(field_type) {
                quote! { xee_xpath::Item::Atomic(xee_xpath::Atomic::Float(f)) => Ok(f.0), }
            } else if is_f64_type(field_type) {
                quote! { xee_xpath::Item::Atomic(xee_xpath::Atomic::Double(f)) => Ok(f.0), }
            } else if is_bool_type(field_type) {
                quote! { xee_xpath::Item::Atomic(xee_xpath::Atomic::Boolean(b)) => Ok(*b), }
            } else {
                quote! {}
            };

            let extract_value_expr = quote! {
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
                                format!("{}", e)
                            )
                        ))
                    )
                })
            };

            if is_f32_type(field_type) || is_f64_type(field_type) || is_bool_type(field_type) {
                quote! {
                    match item {
                        #hot_path
                        _ => { #extract_value_expr },
                    }
                }
            } else {
                extract_value_expr
            }
        }

        _ => {
            panic!("Unsupported attribute at field level: {:?}. This should never happen and is a bug in xee-extract", tag);
        }
    };

    let value_match_arm = if is_option_type(outer_field_type) && is_vec_type(field_type) {
        let default_tokens = default_expr.unwrap_or_else(|| quote! { None });
        quote! {
            if value.is_empty() {
                #default_tokens
            } else {
                Some(value)
            }
        }
    } else if is_option_type(outer_field_type) {
        let default_tokens = default_expr.unwrap_or_else(|| quote! { None });
        quote! { match value {
            Some(value) => Some(value),
            None => { #default_tokens }
        } }
    } else if is_vec_type(outer_field_type) {
        let default_tokens = default_expr.clone().unwrap_or_else(|| quote! { value });
        quote! {
            if value.is_empty() {
                #default_tokens
            } else {
                value
            }
        }
    } else {
        if let Some(expr) = default_expr {
            quote! {
                match value {
                    Some(value) => value,
                    None => { #expr }
                }
            }
        } else {
            quote! {
                match value {
                    Some(value) => value,
                    None => {
                        return Err(xee_extract::Error::FieldExtract(xee_extract::FieldExtractionError {
                            field: #field_name_str,
                            xpath: #xpath_expr_lit,
                            extract_id: #extract_id_match,
                            source: Box::new(xee_extract::NoValueFoundError {}),
                        }));
                    }
                }
            }
        }
    };

    Ok(quote! {
        let #field_name = {
            let query = queries.#query_method(#xpath_expr, |documents, item| {
                #extractor
            })?;

            match query.execute_build_context(documents, |builder| {
                builder.context_item(#context_var.clone());
                builder.variables(variables.clone());
            }) {
                Ok(value) => {
                    #value_match_arm
                },
                Err(inner) => {
                    return Err(xee_extract::Error::FieldExtract(xee_extract::FieldExtractionError {
                        field: #field_name_str,
                        xpath: #xpath_expr_lit,
                        extract_id: #extract_id_match,
                        source: Box::new(inner),
                    }));
                }
            }
        };
    })
}

fn generate_vec_u8_query(
    field_name: &syn::Ident,
    xpath_expr: &str,
    context_var: &proc_macro2::TokenStream,
    field_type: &syn::Type,
    default_expr: Option<proc_macro2::TokenStream>,
) -> syn::Result<proc_macro2::TokenStream> {
    let field_name_token = quote! { #field_name };
    let field_name_str = field_name.to_string();
    let xpath_expr_lit = proc_macro2::Literal::string(xpath_expr);

    let assignment = if is_option_type(field_type) {
        // For Option<Vec<u8>>, return the Option directly
        let default_tokens = default_expr.unwrap_or_else(|| quote! { None });
        quote! {
            let #field_name_token = {
                let query = queries.option(#xpath_expr, |documents, item| {
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
                match query.execute_build_context(documents, |builder| {
                    builder.context_item(#context_var.clone());
                    builder.variables(variables.clone());
                })? {
                    Some(value) => Some(value),
                    None => { #default_tokens }
                }
            };
        }
    } else {
        // For Vec<u8>, unwrap the Option
        if let Some(expr) = default_expr {
            quote! {
                let #field_name_token = {
                    let query = queries.option(#xpath_expr, |documents, item| {
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
                    match query.execute_build_context(documents, |builder| {
                        builder.context_item(#context_var.clone());
                        builder.variables(variables.clone());
                    })? {
                        Some(value) => value,
                        None => { #expr }
                    }
                };
            }
        } else {
            quote! {
                let #field_name_token = {
                    let query = queries.option(#xpath_expr, |documents, item| {
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
                    match query.execute_build_context(documents, |builder| {
                        builder.context_item(#context_var.clone());
                        builder.variables(variables.clone());
                    })? {
                        Some(value) => value,
                        None => {
                            return Err(xee_extract::Error::FieldExtract(xee_extract::FieldExtractionError {
                                field: #field_name_str,
                                xpath: #xpath_expr_lit,
                                extract_id: None,
                                source: Box::new(xee_extract::NoValueFoundError {}),
                            }));
                        }
                    }
                };
            }
        }
    };

    Ok(assignment)
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
        let attrs = vec![attr(quote!(xpath("foo/bar")))];

        let parsed = parse_xee_attrs(&attrs, XeeAttrPosition::Field).unwrap();
        assert_eq!(parsed.len(), 1);
        let attr = &parsed[0];
        assert!(matches!(attr.attr, XeeExtractAttributeTag::Xpath));
        assert_eq!(attr.attr_value, "foo/bar");
        assert_eq!(attr.named_extract, None);
    }

    #[test]
    fn test_xpath_with_alias() {
        let attrs = vec![attr(quote!(xpath("foo/bar", "my_alias")))];

        let parsed = parse_xee_attrs(&attrs, XeeAttrPosition::Field).unwrap();
        assert_eq!(parsed.len(), 1);
        let attr = &parsed[0];
        assert_eq!(attr.attr_value, "foo/bar");
        assert_eq!(attr.named_extract.as_deref(), Some("my_alias"));
    }

    #[test]
    fn test_ns_with_key_value() {
        let attrs = vec![attr(quote!(ns(atom = "http://www.w3.org/2005/Atom")))];

        let parsed = parse_xee_attrs(&attrs, XeeAttrPosition::Struct).unwrap();
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

        let parsed = parse_xee_attrs(&attrs, XeeAttrPosition::Struct).unwrap();
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

        let parsed = parse_xee_attrs(&attrs, XeeAttrPosition::Struct).unwrap();
        assert_eq!(parsed.len(), 2);

        assert!(matches!(parsed[0].attr, XeeExtractAttributeTag::Ns));
        assert!(matches!(parsed[1].attr, XeeExtractAttributeTag::Context));
    }

    #[test]
    fn test_invalid_tag() {
        let attrs = vec![attr(quote!(nonsense("abc")))];

        let result = parse_xee_attrs(&attrs, XeeAttrPosition::Struct);
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_ns_missing_value() {
        let attrs = vec![attr(quote!(ns(atom = 123)))];

        let result = parse_xee_attrs(&attrs, XeeAttrPosition::Struct);
        assert!(result.is_err());
    }
}
