//! Procedural macros for XPath-driven deserialization

use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

/// Derive macro for XPath-driven deserialization
#[proc_macro_derive(XeeExtract, attributes(xpath))]
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
    
    // Generate field extraction code
    let (field_extractions, field_names, field_values) = generate_field_extractions(fields)?;
    
    let expanded = quote! {
        impl #impl_generics crate::XeeExtractDeserialize for #name #ty_generics #where_clause {
            fn deserialize(
                documents: &mut xee_xpath::Documents,
                item: &xee_xpath::Item,
            ) -> Result<Self, xee_extract::Error> {
                use xee_xpath::{Queries, Query};
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
) -> syn::Result<(proc_macro2::TokenStream, Vec<proc_macro2::TokenStream>, Vec<proc_macro2::TokenStream>)> {
    let mut field_names = Vec::new();
    let mut field_values = Vec::new();
    let mut field_extractions = Vec::new();
    
    for field in fields {
        let field_name = field.ident.as_ref()
            .ok_or_else(|| syn::Error::new_spanned(field, "Expected named field"))?;
        
        // Find xpath attribute
        let xpath_expr = field.attrs.iter()
            .find(|attr| attr.path().is_ident("xpath"))
            .and_then(|attr| attr.parse_args::<syn::LitStr>().ok())
            .map(|lit| lit.value())
            .ok_or_else(|| syn::Error::new_spanned(field, "Expected xpath attribute"))?;
        

        // TODO: Validate xpath expression here
        
        field_names.push(quote! { #field_name });
        
        // Generate the extraction code based on the field type
        let extraction = generate_field_extraction(field, &xpath_expr)?;
        field_extractions.push(extraction);
        field_values.push(quote! { #field_name });
    }
    
    Ok((quote! { #(#field_extractions)* }, field_names, field_values))
}

fn generate_field_extraction(
    field: &syn::Field,
    xpath_expr: &str,
) -> syn::Result<proc_macro2::TokenStream> {
    let field_name = field.ident.as_ref().unwrap();
    let field_type = &field.ty;
    
    // Generate the appropriate query based on the field type
    let query_code = if is_option_type(field_type) {
        generate_option_query(xpath_expr, field_type)
    } else if is_vec_type(field_type) {
        generate_vec_query(xpath_expr, field_type)
    } else {
        generate_single_query(xpath_expr, field_type)
    };
    
    Ok(quote! {
        let #field_name = {
            let queries = Queries::default();
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

fn generate_single_query(xpath_expr: &str, field_type: &syn::Type) -> proc_macro2::TokenStream {
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
        query.execute(documents, item)?
    }
}

fn generate_option_query(xpath_expr: &str, field_type: &syn::Type) -> proc_macro2::TokenStream {
    let inner_type = extract_option_inner_type(field_type)
        .expect("Option type should have inner type");
    
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
        query.execute(documents, item)?
    }
}

fn generate_vec_query(xpath_expr: &str, field_type: &syn::Type) -> proc_macro2::TokenStream {
    let inner_type = extract_vec_inner_type(field_type)
        .expect("Vec type should have inner type");
    
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
        query.execute(documents, item)?
    }
} 