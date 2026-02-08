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
fn test_collections() {
    let mut metadata = HashMap::new();
    metadata.insert("author".to_string(), "test".to_string());

    let config = WithCollections {
        items: vec!["a".to_string(), "b".to_string()],
        metadata,
        optional: Some(42),
    };
    let scaffold = config.to_scaffold().unwrap();
    assert_eq!(scaffold, include_str!("collections.toml"));
    
    let deserialized: WithCollections = toml::from_str(&scaffold).unwrap();
    assert_eq!(deserialized, config);
}
