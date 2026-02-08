use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use toml_scaffold::TomlScaffold;

#[derive(Serialize, Deserialize, JsonSchema, TomlScaffold, PartialEq, Debug)]
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

    let deserialized: MultiLineDoc = toml::from_str(&scaffold).unwrap();
    assert_eq!(deserialized, config);
}
