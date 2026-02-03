use serde::{Deserialize, Serialize};
use toml_template::TomlTemplate;

#[derive(Deserialize, Serialize, schemars::JsonSchema, TomlTemplate)]
struct SubstringFields {
    /// User name
    name: String,
    /// Full username
    username: String,
}

#[test]
fn test_substring_field_names() {
    let config = SubstringFields {
        name: "John".to_string(),
        username: "john_doe".to_string(),
    };
    let template = config.to_template();
    assert_eq!(
        template,
        include_str!("fixtures/substring_field_names.toml")
    );
}
