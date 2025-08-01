//! Procedural macros for XPath-driven deserialization
//!
//! This module provides the `Extract` derive macro that allows you to
//! deserialize XML documents into Rust structs using XPath expressions.
//!
use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

/// Derive macro for XPath-driven deserialization
#[proc_macro_derive(Extract, attributes(xpath, extract, xml))]
#[proc_macro_error]
pub fn derive_xee_extract(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    // Parse the input and generate the implementation
    match impl_xee_extract(&input) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

fn impl_xee_extract(input: &DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    // Parse struct fields
    let fields = match &input.data {
        syn::Data::Struct(data) => &data.fields,
        _ => {
            return Err(syn::Error::new_spanned(
                name,
                "Extract can only be derived for structs",
            ))
        }
    };

    // Generate field extraction code
    let (field_extractions, field_names, field_values) = generate_field_extractions(fields)?;

    let expanded = quote! {
        impl #impl_generics xee_extract::Extract for #name #ty_generics #where_clause {
            fn extract(
                documents: &mut xee_xpath::Documents,
                context_item: &xee_xpath::Item,
            ) -> Result<Self, xee_extract::Error> {
                use xee_xpath::{Queries, Query};

                //TODO: Add namespaces to the static context builder
                //TODO: If context is declared using the context attribute, override the provided context_item (this updated context_item is evaulauated using the passed in context_item as the context for the context attribute)

                let static_context_builder = xee_xpath::context::StaticContextBuilder::default();
                let queries = Queries::new(static_context_builder);

                #field_extractions

                Ok(Self {
                    #(#field_names: #field_values,)*
                })
            }
        }
    };

    Ok(expanded)
}

fn generate_field_extractions(
    fields: &syn::Fields,
) -> syn::Result<(
    proc_macro2::TokenStream,
    Vec<proc_macro2::TokenStream>,
    Vec<proc_macro2::TokenStream>,
)> {
    let mut field_names = Vec::new();
    let mut field_values = Vec::new();
    let mut field_extractions = Vec::new();

    for field in fields {
        let field_name = field
            .ident
            .as_ref()
            .ok_or_else(|| syn::Error::new_spanned(field, "Expected named field"))?;

        // Find xpath or extract attribute
        let (xpath_expr, attr_type) = find_xpath_or_extract_attribute(field)?;

        field_names.push(quote! { #field_name });

        // Generate the extraction code based on the field type and attribute
        let extraction = generate_field_extraction(field, &xpath_expr, attr_type)?;
        field_extractions.push(extraction);
        field_values.push(quote! { #field_name });
    }

    Ok((quote! { #(#field_extractions)* }, field_names, field_values))
}

fn find_xpath_or_extract_attribute(field: &syn::Field) -> syn::Result<(String, AttributeType)> {
    // Check for xpath attribute first
    if let Some(attr) = field
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident("xpath"))
    {
        let xpath_expr = attr.parse_args::<syn::LitStr>()?.value();
        return Ok((xpath_expr, AttributeType::XPath));
    }

    // Check for extract attribute
    if let Some(attr) = field
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident("extract"))
    {
        let xpath_expr = attr.parse_args::<syn::LitStr>()?.value();
        return Ok((xpath_expr, AttributeType::Extract));
    }

    // Check for xml attribute
    if let Some(attr) = field.attrs.iter().find(|attr| attr.path().is_ident("xml")) {
        let xpath_expr = attr.parse_args::<syn::LitStr>()?.value();
        return Ok((xpath_expr, AttributeType::Xml));
    }

    Err(syn::Error::new_spanned(
        field,
        "Expected xpath, extract, or xml attribute",
    ))
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum AttributeType {
    XPath,
    Extract,
    Xml,
}

fn generate_field_extraction(
    field: &syn::Field,
    xpath_expr: &str,
    attr_type: AttributeType,
) -> syn::Result<proc_macro2::TokenStream> {
    let field_name = field.ident.as_ref().unwrap();
    let field_type = &field.ty;

    // Generate the appropriate query based on the field type and attribute
    let query_code = if is_option_type(field_type) {
        generate_option_query(xpath_expr, field_type, attr_type)
    } else if is_vec_type(field_type) {
        generate_vec_query(xpath_expr, field_type, attr_type)
    } else {
        generate_single_query(xpath_expr, field_type, attr_type)
    };

    Ok(quote! {
        let #field_name = {
            #query_code
        };
    })
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

fn generate_single_query(
    xpath_expr: &str,
    field_type: &syn::Type,
    attr_type: AttributeType,
) -> proc_macro2::TokenStream {
    match attr_type {
        AttributeType::Extract => {
            quote! {
                let query = queries.one(#xpath_expr, |documents, item| {
                    // For extract attribute, use extract for efficiency
                    use xee_extract::Extract;
                    <#field_type>::extract(documents, item)
                        .map_err(|e| {
                            // Convert xee_extract::Error to xee_interpreter::error::SpannedError
                            match e {
                                xee_extract::Error::DeserializationError(msg) => {
                                    xee_interpreter::error::SpannedError::from(xee_interpreter::error::Error::FORG0001)
                                }
                                _ => xee_interpreter::error::SpannedError::from(xee_interpreter::error::Error::FODC0002)
                            }
                        })
                })?;
                query.execute_build_context(documents, |builder| {
                    builder.context_item(context_item.clone());
                })?
            }
        }
        AttributeType::Xml => {
            quote! {
                let query = queries.one(#xpath_expr, |documents, item| {
                    // For xml attribute, get the full XML serialization
                    match item {
                        xee_xpath::Item::Node(node) => {
                            let xml_str = documents.xot().serialize_xml_string(
                                xot::output::xml::Parameters {
                                    indentation: Default::default(),
                                    ..Default::default()
                                },
                                *node,
                            ).map_err(|_| xee_interpreter::error::SpannedError::from(xee_interpreter::error::Error::FODC0002))?;
                            Ok(xml_str)
                        }
                        _ => {
                            // For non-node items, fall back to string_value
                            let xml_str = item.string_value(documents.xot())?;
                            Ok(xml_str)
                        }
                    }
                })?;
                query.execute_build_context(documents, |builder| {
                    builder.context_item(context_item.clone());
                })?
            }
        }
        AttributeType::XPath => {
            quote! {
                let query = queries.one(#xpath_expr, |documents, item| {
                    use xee_extract::ExtractValue;
                    <#field_type>::deserialize(documents, item)
                        .map_err(|e| {
                            // Convert xee_extract::Error to xee_interpreter::error::SpannedError
                            // For deserialization errors, use FORG0001 (Invalid value for cast/constructor)
                            match e {
                                xee_extract::Error::DeserializationError(msg) => {
                                    xee_interpreter::error::SpannedError::from(xee_interpreter::error::Error::FORG0001)
                                }
                                _ => xee_interpreter::error::SpannedError::from(xee_interpreter::error::Error::FODC0002)
                            }
                        })
                })?;
                query.execute_build_context(documents, |builder| {
                    builder.context_item(context_item.clone());
                })?
            }
        }
    }
}

fn generate_option_query(
    xpath_expr: &str,
    field_type: &syn::Type,
    attr_type: AttributeType,
) -> proc_macro2::TokenStream {
    let inner_type =
        extract_option_inner_type(field_type).expect("Option type should have inner type");

    match attr_type {
        AttributeType::Extract => {
            quote! {
                let query = queries.option(#xpath_expr, |documents, item| {
                    // For extract attribute, use extract for efficiency
                    use xee_extract::Extract;
                    <#inner_type>::extract(documents, item)
                        .map_err(|e| {
                            match e {
                                xee_extract::Error::DeserializationError(msg) => {
                                    xee_interpreter::error::SpannedError::from(xee_interpreter::error::Error::FORG0001)
                                }
                                _ => xee_interpreter::error::SpannedError::from(xee_interpreter::error::Error::FODC0002)
                            }
                        })
                })?;
                query.execute_build_context(documents, |builder| {
                    builder.context_item(context_item.clone());
                })?
            }
        }
        AttributeType::Xml => {
            quote! {
                let query = queries.option(#xpath_expr, |documents, item| {
                    // For xml attribute, get the full XML serialization
                    match item {
                        xee_xpath::Item::Node(node) => {
                            let xml_str = documents.xot().serialize_xml_string(
                                xot::output::xml::Parameters {
                                    indentation: Default::default(),
                                    ..Default::default()
                                },
                                *node,
                            ).map_err(|_| xee_interpreter::error::SpannedError::from(xee_interpreter::error::Error::FODC0002))?;
                            Ok(xml_str)
                        }
                        _ => {
                            // For non-node items, fall back to string_value
                            let xml_str = item.string_value(documents.xot())?;
                            Ok(xml_str)
                        }
                    }
                })?;
                query.execute_build_context(documents, |builder| {
                    builder.context_item(context_item.clone());
                })?
            }
        }
        AttributeType::XPath => {
            quote! {
                let query = queries.option(#xpath_expr, |documents, item| {
                    use xee_extract::ExtractValue;
                    <#inner_type>::deserialize(documents, item)
                        .map_err(|e| {
                            // Convert xee_extract::Error to xee_interpreter::error::SpannedError
                            // For deserialization errors, use FORG0001 (Invalid value for cast/constructor)
                            match e {
                                xee_extract::Error::DeserializationError(msg) => {
                                    xee_interpreter::error::SpannedError::from(xee_interpreter::error::Error::FORG0001)
                                }
                                _ => xee_interpreter::error::SpannedError::from(xee_interpreter::error::Error::FODC0002)
                            }
                        })
                })?;
                query.execute_build_context(documents, |builder| {
                    builder.context_item(context_item.clone());
                })?
            }
        }
    }
}

fn generate_vec_query(
    xpath_expr: &str,
    field_type: &syn::Type,
    attr_type: AttributeType,
) -> proc_macro2::TokenStream {
    let inner_type = extract_vec_inner_type(field_type).expect("Vec type should have inner type");

    match attr_type {
        AttributeType::Extract => {
            quote! {
                let query = queries.many(#xpath_expr, |documents, item| {
                    // For extract attribute, use extract for efficiency
                    use xee_extract::Extract;
                    <#inner_type>::extract(documents, item)
                        .map_err(|e| {
                            match e {
                                xee_extract::Error::DeserializationError(msg) => {
                                    xee_interpreter::error::SpannedError::from(xee_interpreter::error::Error::FORG0001)
                                }
                                _ => xee_interpreter::error::SpannedError::from(xee_interpreter::error::Error::FODC0002)
                            }
                        })
                })?;
                query.execute_build_context(documents, |builder| {
                    builder.context_item(context_item.clone());
                })?
            }
        }
        AttributeType::Xml => {
            quote! {
                let query = queries.many(#xpath_expr, |documents, item| {
                    // For xml attribute, get the full XML serialization
                    match item {
                        xee_xpath::Item::Node(node) => {
                            let xml_str = documents.xot().serialize_xml_string(
                                xot::output::xml::Parameters {
                                    indentation: Default::default(),
                                    ..Default::default()
                                },
                                *node,
                            ).map_err(|_| xee_interpreter::error::SpannedError::from(xee_interpreter::error::Error::FODC0002))?;
                            Ok(xml_str)
                        }
                        _ => {
                            // For non-node items, fall back to string_value
                            let xml_str = item.string_value(documents.xot())?;
                            Ok(xml_str)
                        }
                    }
                })?;
                query.execute_build_context(documents, |builder| {
                    builder.context_item(context_item.clone());
                })?
            }
        }
        AttributeType::XPath => {
            quote! {
                let query = queries.many(#xpath_expr, |documents, item| {
                    use xee_extract::ExtractValue;
                    <#inner_type>::deserialize(documents, item)
                        .map_err(|e| {
                            // Convert xee_extract::Error to xee_interpreter::error::SpannedError
                            // For deserialization errors, use FORG0001 (Invalid value for cast/constructor)
                            match e {
                                xee_extract::Error::DeserializationError(msg) => {
                                    xee_interpreter::error::SpannedError::from(xee_interpreter::error::Error::FORG0001)
                                }
                                _ => xee_interpreter::error::SpannedError::from(xee_interpreter::error::Error::FODC0002)
                            }
                        })
                })?;
                query.execute_build_context(documents, |builder| {
                    builder.context_item(context_item.clone());
                })?
            }
        }
    }
}
