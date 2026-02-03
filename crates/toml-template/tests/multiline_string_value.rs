use serde::{Deserialize, Serialize};
use toml_template::TomlTemplate;

#[derive(Deserialize, Serialize, schemars::JsonSchema, TomlTemplate)]
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
    let template = config.to_template();
    assert_eq!(
        template,
        include_str!("fixtures/multiline_string_value.toml")
    );
}
