//! Procedural macros for XPath-driven deserialization
//!
//! This module provides the `XeeExtract` derive macro that allows you to
//! deserialize XML documents into Rust structs using XPath expressions.
//! 
use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

/// Derive macro for XPath-driven deserialization
#[proc_macro_derive(XeeExtract, attributes(xpath, extract))]
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
        _ => return Err(syn::Error::new_spanned(name, "XeeExtract can only be derived for structs")),
    };
    
    // Generate field extraction code for both regular extract and context-based extract
    let (field_extractions, field_names, field_values) = generate_field_extractions(fields)?;
    let (context_field_extractions, context_field_names, context_field_values) = generate_context_field_extractions(fields)?;
    
    let expanded = quote! {
        impl #impl_generics xee_extract::XeeExtract for #name #ty_generics #where_clause {
            fn extract(
                xml: &str,
            ) -> Result<Self, xee_extract::Error> {
                use xee_xpath::{Queries, Query};
                let queries = Queries::default();
                let mut documents = xee_xpath::Documents::new();
                let doc = documents.add_string_without_uri(xml)?;

                use xee_xpath::Itemable;
                let item = doc.to_item(&mut documents)?;

                #field_extractions
                
                Ok(Self {
                    #(#field_names: #field_values,)*
                })
            }

            fn extract_from_context(
                documents: &mut xee_xpath::Documents,
                context_item: &xee_xpath::Item,
            ) -> Result<Self, xee_extract::Error> {
                use xee_xpath::{Queries, Query};
                let queries = Queries::default();

                #context_field_extractions
                
                Ok(Self {
                    #(#context_field_names: #context_field_values,)*
                })
            }
        }
    };
    
    Ok(expanded)
}

fn generate_field_extractions(
    fields: &syn::Fields,
) -> syn::Result<(proc_macro2::TokenStream, Vec<proc_macro2::TokenStream>, Vec<proc_macro2::TokenStream>)> {
    let mut field_names = Vec::new();
    let mut field_values = Vec::new();
    let mut field_extractions = Vec::new();
    
    for field in fields {
        let field_name = field.ident.as_ref()
            .ok_or_else(|| syn::Error::new_spanned(field, "Expected named field"))?;
        
        // Find xpath or extract attribute
        let (xpath_expr, is_extract) = find_xpath_or_extract_attribute(field)?;
        
        field_names.push(quote! { #field_name });
        
        // Generate the extraction code based on the field type and attribute
        let extraction = generate_field_extraction(field, &xpath_expr, is_extract)?;
        field_extractions.push(extraction);
        field_values.push(quote! { #field_name });
    }
    
    Ok((quote! { #(#field_extractions)* }, field_names, field_values))
}

fn generate_context_field_extractions(
    fields: &syn::Fields,
) -> syn::Result<(proc_macro2::TokenStream, Vec<proc_macro2::TokenStream>, Vec<proc_macro2::TokenStream>)> {
    let mut context_field_names = Vec::new();
    let mut context_field_values = Vec::new();
    let mut context_field_extractions = Vec::new();
    
    for field in fields {
        let field_name = field.ident.as_ref()
            .ok_or_else(|| syn::Error::new_spanned(field, "Expected named field"))?;
        
        // Find xpath or extract attribute
        let (xpath_expr, is_extract) = find_xpath_or_extract_attribute(field)?;
        
        context_field_names.push(quote! { #field_name });
        
        // Generate the extraction code based on the field type and attribute
        let extraction = generate_context_field_extraction(field, &xpath_expr, is_extract)?;
        context_field_extractions.push(extraction);
        context_field_values.push(quote! { #field_name });
    }
    
    Ok((quote! { #(#context_field_extractions)* }, context_field_names, context_field_values))
}

fn find_xpath_or_extract_attribute(field: &syn::Field) -> syn::Result<(String, bool)> {
    // Check for xpath attribute first
    if let Some(attr) = field.attrs.iter().find(|attr| attr.path().is_ident("xpath")) {
        let xpath_expr = attr.parse_args::<syn::LitStr>()?.value();
        return Ok((xpath_expr, false));
    }
    
    // Check for extract attribute
    if let Some(attr) = field.attrs.iter().find(|attr| attr.path().is_ident("extract")) {
        let xpath_expr = attr.parse_args::<syn::LitStr>()?.value();
        return Ok((xpath_expr, true));
    }
    
    Err(syn::Error::new_spanned(field, "Expected either xpath or extract attribute"))
}

fn generate_context_field_extraction(
    field: &syn::Field,
    xpath_expr: &str,
    is_extract: bool,
) -> syn::Result<proc_macro2::TokenStream> {
    let field_name = field.ident.as_ref().unwrap();
    let field_type = &field.ty;
    
    // Generate the appropriate query based on the field type and attribute
    let query_code = if is_option_type(field_type) {
        generate_context_option_query(xpath_expr, field_type, is_extract)
    } else if is_vec_type(field_type) {
        generate_context_vec_query(xpath_expr, field_type, is_extract)
    } else {
        generate_context_single_query(xpath_expr, field_type, is_extract)
    };
    
    Ok(quote! {
        let #field_name = {
            #query_code
        };
    })
}

fn generate_field_extraction(
    field: &syn::Field,
    xpath_expr: &str,
    is_extract: bool,
) -> syn::Result<proc_macro2::TokenStream> {
    let field_name = field.ident.as_ref().unwrap();
    let field_type = &field.ty;
    
    // Generate the appropriate query based on the field type and attribute
    let query_code = if is_option_type(field_type) {
        generate_option_query(xpath_expr, field_type, is_extract)
    } else if is_vec_type(field_type) {
        generate_vec_query(xpath_expr, field_type, is_extract)
    } else {
        generate_single_query(xpath_expr, field_type, is_extract)
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

fn generate_single_query(xpath_expr: &str, field_type: &syn::Type, is_extract: bool) -> proc_macro2::TokenStream {
    if is_extract {
        quote! {
            let query = queries.one(#xpath_expr, |documents, item| {
                // For extract attribute, use extract_from_context for efficiency
                use xee_extract::XeeExtract;
                <#field_type>::extract_from_context(documents, item)
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
            query.execute(&mut documents, &item)?
        }
    } else {
        quote! {
            let query = queries.one(#xpath_expr, |documents, item| {
                use xee_extract::XeeExtractDeserialize;
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
            query.execute(&mut documents, &item)?
        }
    }
}

fn generate_option_query(xpath_expr: &str, field_type: &syn::Type, is_extract: bool) -> proc_macro2::TokenStream {
    let inner_type = extract_option_inner_type(field_type)
        .expect("Option type should have inner type");
    
    if is_extract {
        quote! {
            let query = queries.option(#xpath_expr, |documents, item| {
                // For extract attribute, use extract_from_context for efficiency
                use xee_extract::XeeExtract;
                <#inner_type>::extract_from_context(documents, item)
                    .map_err(|e| {
                        match e {
                            xee_extract::Error::DeserializationError(msg) => {
                                xee_interpreter::error::SpannedError::from(xee_interpreter::error::Error::FORG0001)
                            }
                            _ => xee_interpreter::error::SpannedError::from(xee_interpreter::error::Error::FODC0002)
                        }
                    })
            })?;
            query.execute(&mut documents, &item)?
        }
    } else {
        quote! {
            let query = queries.option(#xpath_expr, |documents, item| {
                use xee_extract::XeeExtractDeserialize;
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
            query.execute(&mut documents, &item)?
        }
    }
}

fn generate_vec_query(xpath_expr: &str, field_type: &syn::Type, is_extract: bool) -> proc_macro2::TokenStream {
    let inner_type = extract_vec_inner_type(field_type)
        .expect("Vec type should have inner type");
    
    if is_extract {
        quote! {
            let query = queries.many(#xpath_expr, |documents, item| {
                // For extract attribute, use extract_from_context for efficiency
                use xee_extract::XeeExtract;
                <#inner_type>::extract_from_context(documents, item)
                    .map_err(|e| {
                        match e {
                            xee_extract::Error::DeserializationError(msg) => {
                                xee_interpreter::error::SpannedError::from(xee_interpreter::error::Error::FORG0001)
                            }
                            _ => xee_interpreter::error::SpannedError::from(xee_interpreter::error::Error::FODC0002)
                        }
                    })
            })?;
            query.execute(&mut documents, &item)?
        }
    } else {
        quote! {
            let query = queries.many(#xpath_expr, |documents, item| {
                use xee_extract::XeeExtractDeserialize;
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
            query.execute(&mut documents, &item)?
        }
    }
} 

fn generate_context_single_query(xpath_expr: &str, field_type: &syn::Type, is_extract: bool) -> proc_macro2::TokenStream {
    if is_extract {
        quote! {
            let query = queries.one(#xpath_expr, |documents, item| {
                // For extract attribute, use extract_from_context for efficiency
                use xee_extract::XeeExtract;
                <#field_type>::extract_from_context(documents, item)
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
    } else {
        quote! {
            let query = queries.one(#xpath_expr, |documents, item| {
                use xee_extract::XeeExtractDeserialize;
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

fn generate_context_option_query(xpath_expr: &str, field_type: &syn::Type, is_extract: bool) -> proc_macro2::TokenStream {
    let inner_type = extract_option_inner_type(field_type)
        .expect("Option type should have inner type");
    
    if is_extract {
        quote! {
            let query = queries.option(#xpath_expr, |documents, item| {
                // For extract attribute, use extract_from_context for efficiency
                use xee_extract::XeeExtract;
                <#inner_type>::extract_from_context(documents, item)
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
    } else {
        quote! {
            let query = queries.option(#xpath_expr, |documents, item| {
                use xee_extract::XeeExtractDeserialize;
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

fn generate_context_vec_query(xpath_expr: &str, field_type: &syn::Type, is_extract: bool) -> proc_macro2::TokenStream {
    let inner_type = extract_vec_inner_type(field_type)
        .expect("Vec type should have inner type");
    
    if is_extract {
        quote! {
            let query = queries.many(#xpath_expr, |documents, item| {
                // For extract attribute, use extract_from_context for efficiency
                use xee_extract::XeeExtract;
                <#inner_type>::extract_from_context(documents, item)
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
    } else {
        quote! {
            let query = queries.many(#xpath_expr, |documents, item| {
                use xee_extract::XeeExtractDeserialize;
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