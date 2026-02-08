use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use toml_scaffold::TomlScaffold;

#[derive(Serialize, Deserialize, JsonSchema, TomlScaffold, PartialEq, Debug)]
struct Database {
    /// Database URL
    url: String,
    /// Connection pool size
    pool_size: usize,
}

#[derive(Serialize, Deserialize, JsonSchema, TomlScaffold, PartialEq, Debug)]
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
    let scaffold = config.to_scaffold().unwrap();
    assert_eq!(scaffold, include_str!("nested_structs.toml"));

    let deserialized: AppConfig = toml::from_str(&scaffold).unwrap();
    assert_eq!(deserialized, config);
}
