use serde::{Deserialize, Serialize};
use toml_template::TomlTemplate;

#[derive(Deserialize, Serialize, schemars::JsonSchema, TomlTemplate)]
struct Config {
    /// Raw JSON value
    data: serde_json::Value,
}

#[test]
fn test_json_value_field() {
    let config = Config {
        data: serde_json::json!({"key": "value", "number": 42}),
    };
    let template = config.to_template().unwrap();
    assert_eq!(template, include_str!("fixtures/json_value_field.toml"));
}

#[test]
fn test_json_value_field_empty() {
    let config = Config {
        data: serde_json::json!({}),
    };
    let template = config.to_template().unwrap();
    assert_eq!(
        template,
        include_str!("fixtures/json_value_field_empty.toml")
    );
}
