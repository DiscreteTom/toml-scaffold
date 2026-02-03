use serde::{Deserialize, Serialize};
use toml_template::TomlTemplate;

#[derive(Deserialize, Serialize, schemars::JsonSchema, TomlTemplate)]
struct Database {
    /// Database URL
    url: String,
    /// Connection pool size
    pool_size: usize,
}

#[derive(Deserialize, Serialize, schemars::JsonSchema, TomlTemplate)]
struct AppConfig {
    /// Application name
    name: String,
    /// Database configuration
    database: Database,
}

#[test]
fn test_nested_structs() {
    let config = AppConfig {
        name: "myapp".to_string(),
        database: Database {
            url: "postgres://localhost".to_string(),
            pool_size: 10,
        },
    };
    let template = config.to_template();
    assert_eq!(template, include_str!("fixtures/nested_structs.toml"));
}
