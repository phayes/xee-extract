extern crate proc_macro;

#[macro_use]
mod error;
mod convert;
mod parse;
mod wrapper;

use quote::quote;
use syn::parse_macro_input;

use parse::XPathFnOptions;
use wrapper::xpath_fn_wrapper;

#[proc_macro_attribute]
pub fn xpath_fn(
    attr: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let options = parse_macro_input!(attr as XPathFnOptions);
    let ast = parse_macro_input!(input as syn::ItemFn);
    let wrapper = xpath_fn_wrapper(&ast, &options).unwrap_or_else(|e| e.into_compile_error());
    quote!(
        #ast
        #wrapper
    )
    .into()
}
