use schemars::JsonSchema;
use serde::Serialize;
use toml_scaffold::TomlScaffold;

#[derive(Serialize, JsonSchema, TomlScaffold)]
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
}

#[test]
fn test_json_value_field_empty() {
    let config = Config {
        data: serde_json::json!({}),
    };
    let scaffold = config.to_scaffold().unwrap();
    assert_eq!(scaffold, include_str!("json_value_field_empty.toml"));
}
