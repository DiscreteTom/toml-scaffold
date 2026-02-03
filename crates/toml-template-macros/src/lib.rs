use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

/// Derive macro for TomlTemplate trait
#[proc_macro_derive(TomlTemplate)]
pub fn derive_toml_template(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let expanded = quote! {
        impl toml_template::TomlTemplate for #name {}
    };

    TokenStream::from(expanded)
}
