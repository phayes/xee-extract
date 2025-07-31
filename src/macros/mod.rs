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
    
    // Parse attributes to extract namespaces and variables
    let namespaces = parse_namespaces(&input.attrs)?;
    let variables = parse_variables(&input.attrs)?;
    
    // Parse struct fields
    let fields = match &input.data {
        syn::Data::Struct(data) => &data.fields,
        _ => return Err(syn::Error::new_spanned(name, "XeeExtract can only be derived for structs")),
    };
    
    // Generate field extraction code
    let field_extractions = generate_field_extractions(fields, &namespaces, &variables)?;
    
    let expanded = quote! {
        impl #impl_generics crate::XeeExtractDeserialize for #name #ty_generics #where_clause {
            fn deserialize(
                documents: &mut xee_xpath::Documents,
                item: &xee_xpath::Item,
            ) -> xee_xpath::error::Result<Self> {
                #field_extractions
                
                Ok(Self {
                    #(#field_names: #field_values,)*
                })
            }
        }
    };
    
    Ok(expanded)
}

fn parse_namespaces(attrs: &[syn::Attribute]) -> syn::Result<Vec<(String, String)>> {
    let mut namespaces = Vec::new();
    
    for attr in attrs {
        if attr.path().is_ident("xpath") {
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("ns") {
                    // Parse namespace declarations
                    meta.parse_nested_meta(|nested_meta| {
                        let key = nested_meta.path.get_ident()
                            .ok_or_else(|| syn::Error::new_spanned(&nested_meta.path, "Expected identifier"))?
                            .to_string();
                        
                        let value = nested_meta.value()?.parse::<syn::LitStr>()?.value();
                        namespaces.push((key, value));
                        Ok(())
                    })?;
                }
                Ok(())
            })?;
        }
    }
    
    Ok(namespaces)
}

fn parse_variables(attrs: &[syn::Attribute]) -> syn::Result<Vec<(String, String)>> {
    let mut variables = Vec::new();
    
    for attr in attrs {
        if attr.path().is_ident("xpath") {
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("var") {
                    // Parse variable declarations
                    meta.parse_nested_meta(|nested_meta| {
                        let key = nested_meta.path.get_ident()
                            .ok_or_else(|| syn::Error::new_spanned(&nested_meta.path, "Expected identifier"))?
                            .to_string();
                        
                        let value = nested_meta.value()?.parse::<syn::LitStr>()?.value();
                        variables.push((key, value));
                        Ok(())
                    })?;
                }
                Ok(())
            })?;
        }
    }
    
    Ok(variables)
}

fn generate_field_extractions(
    fields: &syn::Fields,
    namespaces: &[(String, String)],
    variables: &[(String, String)],
) -> syn::Result<proc_macro2::TokenStream> {
    let mut field_names = Vec::new();
    let mut field_values = Vec::new();
    
    for field in fields {
        let field_name = field.ident.as_ref()
            .ok_or_else(|| syn::Error::new_spanned(field, "Expected named field"))?;
        
        // Find xpath attribute
        let xpath_expr = field.attrs.iter()
            .find(|attr| attr.path().is_ident("xpath"))
            .and_then(|attr| attr.parse_args::<syn::LitStr>().ok())
            .map(|lit| lit.value())
            .ok_or_else(|| syn::Error::new_spanned(field, "Expected xpath attribute"))?;
        
        field_names.push(quote! { #field_name });
        
        // Generate the extraction code based on the field type
        let extraction = generate_field_extraction(field, &xpath_expr, namespaces, variables)?;
        field_values.push(extraction);
    }
    
    Ok(quote! {
        #(#field_extractions)*
    })
}

fn generate_field_extraction(
    field: &syn::Field,
    xpath_expr: &str,
    namespaces: &[(String, String)],
    variables: &[(String, String)],
) -> syn::Result<proc_macro2::TokenStream> {
    let field_name = field.ident.as_ref().unwrap();
    let field_type = &field.ty;
    
    // Create static context with namespaces and variables
    let mut static_context_builder = quote! {
        let mut static_context_builder = xee_xpath::context::StaticContextBuilder::new();
    };
    
    // Add namespaces
    for (prefix, uri) in namespaces {
        static_context_builder = quote! {
            #static_context_builder
            static_context_builder = static_context_builder.namespace(#prefix, #uri);
        };
    }
    
    // Add variables
    for (name, value) in variables {
        static_context_builder = quote! {
            #static_context_builder
            static_context_builder = static_context_builder.variable(#name, #value);
        };
    }
    
    // Create queries
    let queries = quote! {
        let queries = xee_xpath::Queries::new(static_context_builder);
    };
    
    // Generate the appropriate query based on the field type
    let query_code = if is_option_type(field_type) {
        generate_option_query(field_name, xpath_expr)
    } else if is_vec_type(field_type) {
        generate_vec_query(field_name, xpath_expr)
    } else {
        generate_single_query(field_name, xpath_expr)
    };
    
    Ok(quote! {
        #static_context_builder
        #queries
        #query_code
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

fn generate_single_query(field_name: &syn::Ident, xpath_expr: &str) -> proc_macro2::TokenStream {
    quote! {
        let #field_name = {
            let query = queries.one(#xpath_expr, |documents, item| {
                crate::XeeExtractDeserialize::deserialize(documents, item)
            })?;
            query.execute(documents, item)?
        };
    }
}

fn generate_option_query(field_name: &syn::Ident, xpath_expr: &str) -> proc_macro2::TokenStream {
    quote! {
        let #field_name = {
            let query = queries.option(#xpath_expr, |documents, item| {
                crate::XeeExtractDeserialize::deserialize(documents, item)
            })?;
            query.execute(documents, item)?
        };
    }
}

fn generate_vec_query(field_name: &syn::Ident, xpath_expr: &str) -> proc_macro2::TokenStream {
    quote! {
        let #field_name = {
            let query = queries.many(#xpath_expr, |documents, item| {
                crate::XeeExtractDeserialize::deserialize(documents, item)
            })?;
            query.execute(documents, item)?
        };
    }
} 