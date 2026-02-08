use schemars::JsonSchema;
use serde::Serialize;
use toml_scaffold::TomlScaffold;

#[derive(Serialize, JsonSchema, TomlScaffold)]
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
}
