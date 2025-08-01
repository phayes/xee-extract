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
#[proc_macro_derive(Extract, attributes(xpath, extract, xml, ns, context, default_ns))]
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

    // Parse namespace and context attributes
    let namespaces = parse_namespace_attributes(&input.attrs)?;
    let (context_stmt, context_var) = parse_context_attribute(&input.attrs)?;
    let default_namespace = parse_default_namespace_attribute(&input.attrs)?;

    // Generate field extraction code
    let (field_extractions, field_names, field_values) = generate_field_extractions(fields, &context_var)?;

    let expanded = quote! {
        impl #impl_generics xee_extract::Extract for #name #ty_generics #where_clause {
            fn extract(
                documents: &mut xee_xpath::Documents,
                context_item: &xee_xpath::Item,
            ) -> Result<Self, xee_extract::Error> {
                use xee_xpath::{Queries, Query};

                // Build static context with namespaces
                let mut static_context_builder = xee_xpath::context::StaticContextBuilder::default();
                #namespaces
                #default_namespace
                let queries = Queries::new(static_context_builder);

                #context_stmt

                #field_extractions

                Ok(Self {
                    #(#field_names: #field_values,)*
                })
            }
        }
    };

    Ok(expanded)
}

fn parse_namespace_attributes(attrs: &[syn::Attribute]) -> syn::Result<proc_macro2::TokenStream> {
    let ns_attr = attrs.iter().find(|attr| attr.path().is_ident("ns"));
    
    if let Some(attr) = ns_attr {
        let mut namespace_pairs = Vec::new();
        
        // Parse the namespace attribute as a list of key-value pairs
        attr.parse_nested_meta(|meta| {
            let ident = meta.path.get_ident()
                .ok_or_else(|| syn::Error::new_spanned(&meta.path, "Expected namespace prefix"))?;
            let prefix = ident.to_string();
            
            let value = meta.value()?.parse::<syn::LitStr>()?;
            let uri = value.value();
            
            namespace_pairs.push((prefix, uri));
            Ok(())
        })?;
        
        // Generate code to add namespaces to the static context builder
        let namespace_code = namespace_pairs.iter().map(|(prefix, uri)| {
            quote! {
                static_context_builder.add_namespace(#prefix, #uri);
            }
        });
        
        Ok(quote! {
            #(#namespace_code)*
        })
    } else {
        Ok(quote! {})
    }
}

fn parse_context_attribute(attrs: &[syn::Attribute]) -> syn::Result<(proc_macro2::TokenStream, proc_macro2::TokenStream)> {
    let context_attr = attrs.iter().find(|attr| attr.path().is_ident("context"));
    let var = quote! { effective_context_item };
    if let Some(attr) = context_attr {
        let context_expr = attr.parse_args::<syn::LitStr>()?.value();
        Ok((
            quote! {
                let #var = {
                    let context_query = queries.one(#context_expr, |documents, item| Ok(item.clone()))?;
                    context_query.execute_build_context(documents, |builder| {
                        builder.context_item(context_item.clone());
                    })?
                };
            },
            var,
        ))
    } else {
        Ok((quote! { let #var = context_item; }, var))
    }
}

fn parse_default_namespace_attribute(attrs: &[syn::Attribute]) -> syn::Result<proc_macro2::TokenStream> {
    let default_ns_attr = attrs.iter().find(|attr| attr.path().is_ident("default_ns"));
    
    if let Some(attr) = default_ns_attr {
        let namespace_uri = attr.parse_args::<syn::LitStr>()?.value();
        Ok(quote! {
            static_context_builder.default_element_namespace(#namespace_uri);
        })
    } else {
        Ok(quote! {})
    }
}

fn generate_field_extractions(
    fields: &syn::Fields,
    context_var: &proc_macro2::TokenStream,
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
        let extraction = generate_field_extraction(field, &xpath_expr, attr_type, context_var)?;
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
    context_var: &proc_macro2::TokenStream,
) -> syn::Result<proc_macro2::TokenStream> {
    let field_name = field.ident.as_ref().unwrap();
    let field_type = &field.ty;

    // Check if this is Vec<u8> or Option<Vec<u8>> for special binary handling (only for XPath attribute)
    if (is_vec_u8_type(field_type) || is_option_vec_u8_type(field_type)) && attr_type == AttributeType::XPath {
        return generate_vec_u8_query(field_name, xpath_expr, field_type, context_var);
    }

    // Generate the appropriate query based on the field type and attribute
    let query_code = if is_option_type(field_type) {
        let inner_type = extract_option_inner_type(field_type).expect("Option type should have inner type");
        generate_unified_query(xpath_expr, inner_type, attr_type, context_var, quote! { option })
    } else if is_vec_type(field_type) {
        let inner_type = extract_vec_inner_type(field_type).expect("Vec type should have inner type");
        generate_unified_query(xpath_expr, inner_type, attr_type, context_var, quote! { many })
    } else {
        generate_unified_query(xpath_expr, field_type, attr_type, context_var, quote! { one })
    };

    let field_name_token = quote! { #field_name };
    Ok(quote! {
        let #field_name_token = {
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
    attr_type: AttributeType,
    context_var: &proc_macro2::TokenStream,
    query_method: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    match attr_type {
        AttributeType::Extract => {
            quote! {
                let query = queries.#query_method(#xpath_expr, |documents, item| {
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
                    builder.context_item(#context_var.clone());
                })?
            }
        }
        AttributeType::Xml => {
            quote! {
                let query = queries.#query_method(#xpath_expr, |documents, item| {
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
                    builder.context_item(#context_var.clone());
                })?
            }
        }
        AttributeType::XPath => {
            quote! {
                let query = queries.#query_method(#xpath_expr, |documents, item| {
                    use xee_extract::ExtractValue;
                    <#field_type>::extract_value(documents, item)
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
                    builder.context_item(#context_var.clone());
                })?
            }
        }
    }
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
