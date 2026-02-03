use schemars::JsonSchema;
use serde::Serialize;
use toml_scaffold::TomlScaffold;

#[derive(Serialize, JsonSchema, TomlScaffold)]
struct DeepInner {
    /// Inner value
    value: String,
}

#[derive(Serialize, JsonSchema, TomlScaffold)]
struct DeepMiddle {
    /// Inner configuration
    inner: DeepInner,
}

#[derive(Serialize, JsonSchema, TomlScaffold)]
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
