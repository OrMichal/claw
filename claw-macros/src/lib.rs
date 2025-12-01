use proc_macro::TokenStream;
use quote::quote;
use syn::{LitStr, parse_macro_input};

#[proc_macro]
pub fn router(_: TokenStream) -> TokenStream {
    let tokens = quote! {
        let a = core::
    };

    proc_macro::TokenStream::from(tokens)
}

#[proc_macro]
pub fn template(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitStr);
    let html_str = input.value();

    let tokens = quote! {
        Box::new()
    };

    TokenStream::from(tokens)
}
