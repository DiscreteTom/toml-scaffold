use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use toml_scaffold::TomlScaffold;

#[derive(Serialize, Deserialize, JsonSchema, TomlScaffold, PartialEq, Debug)]
struct Inner {
    /// Inner value
    value: String,
}

#[derive(Serialize, Deserialize, JsonSchema, TomlScaffold, PartialEq, Debug)]
struct Middle {
    /// Middle field
    middle_field: String,
    /// Nested inner with dots
    #[serde(rename = "inner.with.dots")]
    inner_with_dots: Inner,
}

#[derive(Serialize, Deserialize, JsonSchema, TomlScaffold, PartialEq, Debug)]
struct Config {
    /// Regular field
    regular: String,
    /// Nested table with dots in name
    #[serde(rename = "table.with.dots")]
    table_with_dots: Middle,
}

#[test]
fn test_nested_with_special_chars() {
    let config = Config {
        regular: "value".to_string(),
        table_with_dots: Middle {
            middle_field: "middle".to_string(),
            inner_with_dots: Inner {
                value: "nested".to_string(),
            },
        },
    };
    let scaffold = config.to_scaffold().unwrap();
    assert_eq!(scaffold, include_str!("nested_with_special_chars.toml"));

    let deserialized: Config = toml::from_str(&scaffold).unwrap();
    assert_eq!(deserialized, config);
}
