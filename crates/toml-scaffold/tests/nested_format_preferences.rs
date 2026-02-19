use schemars::JsonSchema;
use serde::Serialize;
use serde_json::json;
use toml_scaffold::TomlScaffold;

#[derive(Serialize, JsonSchema, TomlScaffold)]
struct LlmConfig {
    base_url: String,
    #[format = "*dotted"]
    body: serde_json::Value,
}

#[derive(Serialize, JsonSchema, TomlScaffold)]
struct Config {
    name: String,
    llm: LlmConfig,
}

#[test]
fn test_nested_format_preferences() {
    let config = Config {
        name: "test".to_string(),
        llm: LlmConfig {
            base_url: "https://example.com".to_string(),
            body: json!({
                "parallel_tool_calls": true,
                "reasoning": {
                    "effort": "medium"
                }
            }),
        },
    };

    let scaffold = config.to_scaffold().unwrap();
    assert_eq!(scaffold, include_str!("nested_format_preferences.toml"));
}
