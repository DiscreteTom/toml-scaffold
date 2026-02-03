use serde::{Deserialize, Serialize};
use toml_template::TomlTemplate;

#[derive(Deserialize, Serialize, schemars::JsonSchema, TomlTemplate)]
struct DeepInner {
    /// Inner value
    value: String,
}

#[derive(Deserialize, Serialize, schemars::JsonSchema, TomlTemplate)]
struct DeepMiddle {
    /// Inner configuration
    inner: DeepInner,
}

#[derive(Deserialize, Serialize, schemars::JsonSchema, TomlTemplate)]
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
    let template = config.to_template().unwrap();
    assert_eq!(template, include_str!("fixtures/deeply_nested.toml"));
}
