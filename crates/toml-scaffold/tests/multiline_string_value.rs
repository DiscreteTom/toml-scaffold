use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use toml_scaffold::TomlScaffold;

#[derive(Serialize, Deserialize, JsonSchema, TomlScaffold, PartialEq, Debug)]
struct MultiLineString {
    /// A description field
    description: String,
    /// Another field
    value: i32,
}

#[test]
fn test_multiline_string_value() {
    let config = MultiLineString {
        description: "\nThis is a test\nvalue = 123\nmore text\n".to_string(),
        value: 42,
    };
    let scaffold = config.to_scaffold().unwrap();
    assert_eq!(scaffold, include_str!("multiline_string_value.toml"));
    let deserialized: MultiLineString = toml::from_str(&scaffold).unwrap();
    assert_eq!(deserialized, config);
}
