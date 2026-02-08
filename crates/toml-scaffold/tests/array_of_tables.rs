use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use toml_scaffold::TomlScaffold;

#[derive(Serialize, Deserialize, JsonSchema, TomlScaffold, PartialEq, Debug)]
struct Item {
    /// Item name
    name: String,
}

#[derive(Serialize, Deserialize, JsonSchema, TomlScaffold, PartialEq, Debug)]
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
    
    let deserialized: WithArray = toml::from_str(&scaffold).unwrap();
    assert_eq!(deserialized, config);
}
