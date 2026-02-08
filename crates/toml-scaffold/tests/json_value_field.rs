use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use toml_scaffold::TomlScaffold;

#[derive(Serialize, Deserialize, JsonSchema, TomlScaffold, PartialEq, Debug)]
struct Config {
    /// Raw JSON value
    data: serde_json::Value,
}

#[test]
fn test_json_value_field() {
    let config = Config {
        data: serde_json::json!({"key": "value", "number": 42}),
    };
    let scaffold = config.to_scaffold().unwrap();
    assert_eq!(scaffold, include_str!("json_value_field.toml"));
    
    let deserialized: Config = toml::from_str(&scaffold).unwrap();
    assert_eq!(deserialized, config);
}

#[test]
fn test_json_value_field_empty() {
    let config = Config {
        data: serde_json::json!({}),
    };
    let scaffold = config.to_scaffold().unwrap();
    assert_eq!(scaffold, include_str!("json_value_field_empty.toml"));
    
    let deserialized: Config = toml::from_str(&scaffold).unwrap();
    assert_eq!(deserialized, config);
}
