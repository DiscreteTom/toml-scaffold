use serde::{Deserialize, Serialize};
use toml_scaffold::TomlScaffold;

#[derive(Deserialize, Serialize, schemars::JsonSchema, TomlScaffold)]
struct Item {
    /// Item name
    name: String,
}

#[derive(Deserialize, Serialize, schemars::JsonSchema, TomlScaffold)]
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
    let scaffold = config.to_scaffold().unwrap();
    assert_eq!(scaffold, include_str!("array_of_tables.toml"));
}
