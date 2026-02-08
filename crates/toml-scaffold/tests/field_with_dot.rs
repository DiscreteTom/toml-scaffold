use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use toml_scaffold::TomlScaffold;

#[derive(Serialize, Deserialize, JsonSchema, TomlScaffold, PartialEq, Debug)]
struct Config {
    /// Field with a dot in its name
    #[serde(rename = "field.with.dots")]
    field_with_dots: String,
    /// Regular field
    regular: String,
}

#[test]
fn test_field_with_dot() {
    let config = Config {
        field_with_dots: "value".to_string(),
        regular: "normal".to_string(),
    };
    let scaffold = config.to_scaffold().unwrap();
    assert_eq!(scaffold, include_str!("field_with_dot.toml"));

    let deserialized: Config = toml::from_str(&scaffold).unwrap();
    assert_eq!(deserialized, config);
}
