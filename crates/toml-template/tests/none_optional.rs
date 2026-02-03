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
fn test_none_optional() {
    let config = WithCollections {
        items: vec![],
        metadata: HashMap::new(),
        optional: None,
    };
    let template = config.to_template().unwrap();
    assert_eq!(template, include_str!("fixtures/none_optional.toml"));
}
