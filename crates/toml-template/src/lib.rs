mod format;
mod schema;

use schemars::JsonSchema;
use serde::Serialize;
pub use toml_template_macros::TomlTemplate;

/// Trait for generating TOML templates from Rust structs
pub trait TomlTemplate: Serialize + JsonSchema {
    /// Generate a template from an actual struct instance
    fn to_template(&self) -> String {
        let schema = schemars::schema_for!(Self);
        let toml_str = toml::to_string(self).unwrap();
        let toml_value: toml::Value = toml::from_str(&toml_str).unwrap();
        let comments = schema::extract_all_comments(&schema, "");
        let result = format::format_with_comments(&toml_value, &comments, "");
        // Rule 14: Always end file with a single newline
        format!("{}\n", result.trim_end())
    }
}
