use serde::{Deserialize, Serialize};
use toml_scaffold::TomlScaffold;

#[derive(Deserialize, Serialize, schemars::JsonSchema, TomlScaffold)]
struct MultiLineString {
    /// A description field
    description: String,
    /// Another field
    value: i32,
}

#[test]
fn test_multiline_string_value() {
    let config = MultiLineString {
        description: "This is a test\nvalue = 123\nmore text".to_string(),
        value: 42,
    };
    let scaffold = config.to_scaffold().unwrap();
    assert_eq!(scaffold, include_str!("multiline_string_value.toml"));
}
