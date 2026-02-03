use serde::{Deserialize, Serialize};
use toml_scaffold::TomlScaffold;

#[derive(Deserialize, Serialize, schemars::JsonSchema, TomlScaffold)]
struct DeepInner {
    /// Inner value
    value: String,
}

#[derive(Deserialize, Serialize, schemars::JsonSchema, TomlScaffold)]
struct DeepMiddle {
    /// Inner configuration
    inner: DeepInner,
}

#[derive(Deserialize, Serialize, schemars::JsonSchema, TomlScaffold)]
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
}
