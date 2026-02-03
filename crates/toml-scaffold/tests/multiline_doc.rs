use schemars::JsonSchema;
use serde::Serialize;
use toml_scaffold::TomlScaffold;

#[derive(Serialize, JsonSchema, TomlScaffold)]
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
    let scaffold = config.to_scaffold().unwrap();
    assert_eq!(scaffold, include_str!("multiline_doc.toml"));
}
