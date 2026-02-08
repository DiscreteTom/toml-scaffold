use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use toml_scaffold::TomlScaffold;

#[derive(Serialize, Deserialize, JsonSchema, TomlScaffold, PartialEq, Debug)]
struct Config {
    /// Required field
    name: String,
    /// Optional nested struct
    database: Option<Database>,
}

#[derive(Serialize, Deserialize, JsonSchema, TomlScaffold, PartialEq, Debug)]
struct Database {
    /// Database URL
    url: String,
    /// Optional port
    port: Option<u16>,
}

#[test]
fn test_optional_nested_struct_none() {
    let config = Config {
        name: "app".to_string(),
        database: None,
    };
    let scaffold = config.to_scaffold().unwrap();
    assert_eq!(scaffold, include_str!("optional_nested_struct_none.toml"));
    let deserialized: Config = toml::from_str(&scaffold).unwrap();
    assert_eq!(deserialized, config);
}

#[test]
fn test_optional_nested_struct_some() {
    let config = Config {
        name: "app".to_string(),
        database: Some(Database {
            url: "localhost".to_string(),
            port: None,
        }),
    };
    let scaffold = config.to_scaffold().unwrap();
    assert_eq!(scaffold, include_str!("optional_nested_struct_some.toml"));
    let deserialized: Config = toml::from_str(&scaffold).unwrap();
    assert_eq!(deserialized, config);
}
