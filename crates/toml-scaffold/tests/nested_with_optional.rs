use schemars::JsonSchema;
use serde::Serialize;
use toml_scaffold::TomlScaffold;

#[derive(Serialize, JsonSchema, TomlScaffold)]
struct Config {
    /// Worker configuration
    worker: WorkerConfig,
}

#[derive(Serialize, JsonSchema, TomlScaffold)]
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
}
