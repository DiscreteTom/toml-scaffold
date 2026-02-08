use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use toml_scaffold::TomlScaffold;

#[derive(Serialize, Deserialize, JsonSchema, TomlScaffold, PartialEq, Debug)]
struct Config {
    /// Original field name
    #[serde(rename = "renamed-field")]
    original_field: String,
    /// Normal field
    normal: String,
}

#[test]
fn test_serde_rename() {
    let config = Config {
        original_field: "value".to_string(),
        normal: "data".to_string(),
    };
    let scaffold = config.to_scaffold().unwrap();
    assert_eq!(scaffold, include_str!("serde_rename.toml"));
    let deserialized: Config = toml::from_str(&scaffold).unwrap();
    assert_eq!(deserialized, config);
}
