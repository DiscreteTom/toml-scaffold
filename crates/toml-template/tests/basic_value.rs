use serde::{Deserialize, Serialize};
use toml_template::TomlTemplate;

#[derive(Deserialize, Serialize, schemars::JsonSchema, TomlTemplate)]
struct Config {
    /// Server host address
    host: String,
    /// Server port
    port: u16,
}

#[test]
fn test_basic_value_template() {
    let config = Config {
        host: "localhost".to_string(),
        port: 8080,
    };
    let template = config.to_template().unwrap();
    assert_eq!(template, include_str!("basic_value.toml"));
}
