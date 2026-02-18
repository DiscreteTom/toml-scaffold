#![doc = include_str!("../README.md")]

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Expr, ExprLit, Fields, Lit};

/// Derive macro for TomlScaffold trait
#[proc_macro_derive(TomlScaffold, attributes(format))]
pub fn derive_toml_scaffold(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let format_attrs = extract_format_attributes(&input.data);

    let field_names: Vec<_> = format_attrs.iter().map(|(k, _)| k).collect();
    let format_values: Vec<_> = format_attrs.iter().map(|(_, v)| v).collect();

    let expanded = quote! {
        impl toml_scaffold::TomlScaffold for #name {
            fn format_preferences() -> ::std::collections::HashMap<String, String> {
                let mut map = ::std::collections::HashMap::new();
                #(map.insert(#field_names.to_string(), #format_values.to_string());)*
                map
            }
        }
    };

    TokenStream::from(expanded)
}

fn extract_format_attributes(data: &Data) -> Vec<(String, String)> {
    let mut attrs = Vec::new();

    if let Data::Struct(data_struct) = data {
        if let Fields::Named(fields) = &data_struct.fields {
            for field in &fields.named {
                let field_name = field.ident.as_ref().unwrap().to_string();

                for attr in &field.attrs {
                    if !attr.path().is_ident("format") {
                        continue;
                    }

                    // Try parsing as name-value: #[format = "value"]
                    if let syn::Meta::NameValue(nv) = &attr.meta {
                        if let Expr::Lit(ExprLit {
                            lit: Lit::Str(lit_str),
                            ..
                        }) = &nv.value
                        {
                            attrs.push((field_name.clone(), lit_str.value()));
                            continue;
                        }
                    }

                    // Try parsing as function-like: #[format("value")]
                    if let Ok(Expr::Lit(ExprLit {
                        lit: Lit::Str(lit_str),
                        ..
                    })) = attr.parse_args()
                    {
                        attrs.push((field_name.clone(), lit_str.value()));
                    }
                }
            }
        }
    }

    attrs
}
