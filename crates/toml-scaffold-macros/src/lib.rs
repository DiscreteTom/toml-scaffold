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
    let fields = extract_fields(&input.data);

    let field_names: Vec<_> = format_attrs.iter().map(|(k, _)| k).collect();
    let format_values: Vec<_> = format_attrs.iter().map(|(_, v)| v).collect();

    let nested_field_names: Vec<_> = fields.iter().map(|(name, _)| name).collect();
    let nested_field_types: Vec<_> = fields.iter().map(|(_, ty)| ty).collect();

    let expanded = quote! {
        impl toml_scaffold::TomlScaffold for #name {
            fn format_preferences() -> ::std::collections::HashMap<toml_scaffold::FieldPath, String> {
                let mut map = ::std::collections::HashMap::new();

                // Direct format attributes
                #(map.insert(
                    toml_scaffold::FieldPath::from_vec(vec![#field_names.to_string()]),
                    #format_values.to_string()
                );)*

                // Collect from nested types
                #(
                    for (nested_path, nested_value) in <#nested_field_types>::format_preferences() {
                        let mut full_path = toml_scaffold::FieldPath::from_vec(vec![#nested_field_names.to_string()]);
                        for i in 0..nested_path.len() {
                            if let Some(segment) = nested_path.get(i) {
                                full_path.push(segment.clone());
                            }
                        }
                        map.insert(full_path, nested_value);
                    }
                )*

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

fn extract_fields(data: &Data) -> Vec<(String, syn::Type)> {
    let mut fields = Vec::new();

    if let Data::Struct(data_struct) = data {
        if let Fields::Named(named_fields) = &data_struct.fields {
            for field in &named_fields.named {
                let field_name = field.ident.as_ref().unwrap().to_string();
                let field_type = field.ty.clone();
                fields.push((field_name, field_type));
            }
        }
    }

    fields
}
