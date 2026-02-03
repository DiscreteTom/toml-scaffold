use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use toml_template::TomlTemplate;

#[derive(Deserialize, Serialize, schemars::JsonSchema, TomlTemplate)]
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
    let template = config.to_template();
    assert_eq!(template, include_str!("fixtures/collections.toml"));
}
