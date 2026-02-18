use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::json;
use toml_scaffold::TomlScaffold;

#[derive(Serialize, Deserialize, JsonSchema, TomlScaffold, PartialEq, Debug)]
struct Config {
    /// Application name
    name: String,
    /// Body configuration (dotted format)
    #[format = "dotted"]
    body: serde_json::Value,
}

#[test]
fn test_format_json_value_dotted() {
    let config = Config {
        name: "myapp".to_string(),
        body: json!({
            "key1": 123,
            "key2": "value"
        }),
    };
    let scaffold = config.to_scaffold().unwrap();
    assert_eq!(scaffold, include_str!("format_json_value.toml"));

    let deserialized: Config = toml::from_str(&scaffold).unwrap();
    assert_eq!(deserialized, config);
}

#[test]
fn test_format_json_value_dotted_nested() {
    let config = Config {
        name: "myapp".to_string(),
        body: json!({
            "key1": 123,
            "key2": {
                "key3": 456
            }
        }),
    };
    let scaffold = config.to_scaffold().unwrap();
    assert_eq!(scaffold, include_str!("format_json_value_nested.toml"));

    let deserialized: Config = toml::from_str(&scaffold).unwrap();
    assert_eq!(deserialized, config);
}

#[derive(Serialize, Deserialize, JsonSchema, TomlScaffold, PartialEq, Debug)]
struct ConfigDottedNested {
    /// Application name
    name: String,
    /// Body configuration (dotted-nested format)
    #[format = "dotted-nested"]
    body: serde_json::Value,
}

#[test]
fn test_format_json_value_dotted_nested_cascade() {
    let config = ConfigDottedNested {
        name: "myapp".to_string(),
        body: json!({
            "key1": 123,
            "key2": {
                "key3": 456,
                "key4": {
                    "key5": 789
                }
            }
        }),
    };
    let scaffold = config.to_scaffold().unwrap();
    assert_eq!(
        scaffold,
        include_str!("format_json_value_nested_cascade.toml")
    );

    let deserialized: ConfigDottedNested = toml::from_str(&scaffold).unwrap();
    assert_eq!(deserialized, config);
}

#[derive(Serialize, Deserialize, JsonSchema, TomlScaffold, PartialEq, Debug)]
struct ConfigChildDotted {
    /// Application name
    name: String,
    /// Body configuration (child dotted format)
    #[format = "*dotted"]
    body: serde_json::Value,
}

#[test]
fn test_format_json_value_child_dotted() {
    let config = ConfigChildDotted {
        name: "myapp".to_string(),
        body: json!({
            "key1": 123,
            "key2": {
                "key3": 456
            }
        }),
    };
    let scaffold = config.to_scaffold().unwrap();
    assert_eq!(
        scaffold,
        include_str!("format_json_value_child_dotted.toml")
    );

    let deserialized: ConfigChildDotted = toml::from_str(&scaffold).unwrap();
    assert_eq!(deserialized, config);
}
