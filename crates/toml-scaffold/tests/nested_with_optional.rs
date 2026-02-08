use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use toml_scaffold::TomlScaffold;

#[derive(Serialize, Deserialize, JsonSchema, TomlScaffold, PartialEq, Debug)]
struct Config {
    /// Worker configuration
    worker: WorkerConfig,
}

#[derive(Serialize, Deserialize, JsonSchema, TomlScaffold, PartialEq, Debug)]
struct WorkerConfig {
    /// Maximum files per task
    max_files: usize,
    /// Optional parallel workers
    max_parallel: Option<usize>,
}

#[test]
fn test_nested_with_optional() {
    let config = Config {
        worker: WorkerConfig {
            max_files: 5,
            max_parallel: None,
        },
    };
    let scaffold = config.to_scaffold().unwrap();
    assert_eq!(scaffold, include_str!("nested_with_optional.toml"));
    let deserialized: Config = toml::from_str(&scaffold).unwrap();
    assert_eq!(deserialized, config);
}
