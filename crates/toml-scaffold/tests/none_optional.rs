use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use toml_scaffold::TomlScaffold;

#[derive(Serialize, Deserialize, JsonSchema, TomlScaffold, PartialEq, Debug)]
struct WithCollections {
    /// List of items
    items: Vec<String>,
    /// Key-value pairs
    metadata: HashMap<String, String>,
    /// Optional field
    optional: Option<i32>,
}

#[test]
fn test_none_optional() {
    let config = WithCollections {
        items: vec![],
        metadata: HashMap::new(),
        optional: None,
    };
    let scaffold = config.to_scaffold().unwrap();
    assert_eq!(scaffold, include_str!("none_optional.toml"));
    let deserialized: WithCollections = toml::from_str(&scaffold).unwrap();
    assert_eq!(deserialized, config);
}
