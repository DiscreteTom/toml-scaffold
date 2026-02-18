use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use toml_scaffold::TomlScaffold;

#[derive(Serialize, Deserialize, JsonSchema, TomlScaffold, PartialEq, Debug)]
struct Config {
    /// List of ports (multiline format)
    #[format = "multiline"]
    ports: Vec<u16>,
    /// List of tags (default inline format)
    tags: Vec<String>,
}

#[test]
fn test_format_multiline_array() {
    let config = Config {
        ports: vec![8080, 8081],
        tags: vec!["web".to_string(), "api".to_string()],
    };
    let scaffold = config.to_scaffold().unwrap();
    assert_eq!(scaffold, include_str!("format_multiline_array.toml"));

    let deserialized: Config = toml::from_str(&scaffold).unwrap();
    assert_eq!(deserialized, config);
}
