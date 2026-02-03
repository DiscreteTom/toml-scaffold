use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

/// Derive macro for TomlScaffold trait
#[proc_macro_derive(TomlScaffold)]
pub fn derive_toml_scaffold(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let expanded = quote! {
        impl toml_scaffold::TomlScaffold for #name {}
    };

    TokenStream::from(expanded)
}
