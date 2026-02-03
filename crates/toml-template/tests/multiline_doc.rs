use serde::{Deserialize, Serialize};
use toml_template::TomlTemplate;

#[derive(Deserialize, Serialize, schemars::JsonSchema, TomlTemplate)]
struct MultiLineDoc {
    /// This is a field
    ///
    /// with multiple lines
    ///
    /// of documentation
    field: String,
}

#[test]
fn test_multiline_doc() {
    let config = MultiLineDoc {
        field: "value".to_string(),
    };
    let template = config.to_template().unwrap();
    assert_eq!(template, include_str!("fixtures/multiline_doc.toml"));
}
