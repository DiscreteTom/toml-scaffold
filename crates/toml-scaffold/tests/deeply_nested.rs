use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use toml_scaffold::TomlScaffold;

#[derive(Serialize, Deserialize, JsonSchema, TomlScaffold, PartialEq, Debug)]
struct DeepInner {
    /// Inner value
    value: String,
}

#[derive(Serialize, Deserialize, JsonSchema, TomlScaffold, PartialEq, Debug)]
struct DeepMiddle {
    /// Inner configuration
    inner: DeepInner,
}

#[derive(Serialize, Deserialize, JsonSchema, TomlScaffold, PartialEq, Debug)]
struct DeepOuter {
    /// Middle configuration
    middle: DeepMiddle,
}

#[test]
fn test_deeply_nested() {
    let config = DeepOuter {
        middle: DeepMiddle {
            inner: DeepInner {
                value: "test".to_string(),
            },
        },
    };
    let scaffold = config.to_scaffold().unwrap();
    assert_eq!(scaffold, include_str!("deeply_nested.toml"));
    
    let deserialized: DeepOuter = toml::from_str(&scaffold).unwrap();
    assert_eq!(deserialized, config);
}
