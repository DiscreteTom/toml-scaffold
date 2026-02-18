use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use toml_scaffold::TomlScaffold;

#[derive(Serialize, Deserialize, JsonSchema, TomlScaffold, PartialEq, Debug)]
struct Database {
    /// Database host
    host: String,
    /// Database port
    port: u16,
}

#[derive(Serialize, Deserialize, JsonSchema, TomlScaffold, PartialEq, Debug)]
struct Server {
    /// Server host
    host: String,
    /// Server port
    port: u16,
}

#[derive(Serialize, Deserialize, JsonSchema, TomlScaffold, PartialEq, Debug)]
struct Config {
    /// Application name
    name: String,
    /// Database configuration (inline format)
    #[format = "inline"]
    database: Database,
    /// Server configuration (dotted format)
    #[format = "dotted"]
    server: Server,
}

#[test]
fn test_format_inline_and_dotted() {
    let config = Config {
        name: "myapp".to_string(),
        database: Database {
            host: "localhost".to_string(),
            port: 5432,
        },
        server: Server {
            host: "0.0.0.0".to_string(),
            port: 8080,
        },
    };
    let scaffold = config.to_scaffold().unwrap();
    assert_eq!(scaffold, include_str!("format_inline_dotted.toml"));

    let deserialized: Config = toml::from_str(&scaffold).unwrap();
    assert_eq!(deserialized, config);
}
