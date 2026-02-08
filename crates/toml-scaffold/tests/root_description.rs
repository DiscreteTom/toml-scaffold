use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use toml_scaffold::TomlScaffold;

/// This is the main configuration
/// for the application
#[derive(Serialize, Deserialize, JsonSchema, TomlScaffold, PartialEq, Debug)]
struct Config {
    /// Server host
    host: String,
    /// Server port
    port: u16,
}

#[test]
fn test_root_description() {
    let config = Config {
        host: "localhost".to_string(),
        port: 8080,
    };
    let scaffold = config.to_scaffold().unwrap();
    assert_eq!(scaffold, include_str!("root_description.toml"));

    let deserialized: Config = toml::from_str(&scaffold).unwrap();
    assert_eq!(deserialized, config);
}
