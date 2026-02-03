use serde::{Deserialize, Serialize};
use toml_template::TomlTemplate;

#[derive(Deserialize, Serialize, schemars::JsonSchema, TomlTemplate)]
struct Item {
    /// Item name
    name: String,
}

#[derive(Deserialize, Serialize, schemars::JsonSchema, TomlTemplate)]
struct WithArray {
    /// Array of items
    items: Vec<Item>,
}

#[test]
fn test_array_of_tables() {
    let config = WithArray {
        items: vec![
            Item {
                name: "first".to_string(),
            },
            Item {
                name: "second".to_string(),
            },
        ],
    };
    let template = config.to_template().unwrap();
    assert_eq!(template, include_str!("fixtures/array_of_tables.toml"));
}
