use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use toml_scaffold::TomlScaffold;

#[derive(Serialize, Deserialize, JsonSchema, TomlScaffold, PartialEq, Debug)]
struct Section1 {
    field1: String,
}

#[derive(Serialize, Deserialize, JsonSchema, TomlScaffold, PartialEq, Debug)]
struct Section2 {
    field2: String,
}

#[derive(Serialize, Deserialize, JsonSchema, TomlScaffold, PartialEq, Debug)]
struct Config {
    section1: Section1,
    section2: Section2,
}

#[test]
fn test_no_duplicate_section_separator() {
    let config = Config {
        section1: Section1 {
            field1: "value1".to_string(),
        },
        section2: Section2 {
            field2: "value2".to_string(),
        },
    };
    let scaffold = config.to_scaffold().unwrap();
    assert_eq!(scaffold, include_str!("no_duplicate_section_separator.toml"));

    let deserialized: Config = toml::from_str(&scaffold).unwrap();
    assert_eq!(deserialized, config);
}
