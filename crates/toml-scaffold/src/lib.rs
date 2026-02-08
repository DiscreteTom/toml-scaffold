#![doc = include_str!("../../../README.md")]

mod format;
mod schema;

use schemars::JsonSchema;
use serde::Serialize;
pub use toml_scaffold_macros::TomlScaffold;

/// Trait for generating TOML scaffolds from Rust structs
pub trait TomlScaffold: Serialize + JsonSchema {
    /// Generate a scaffold from an actual struct instance
    ///
    /// Returns an error if serialization fails
    fn to_scaffold(&self) -> Result<String, toml::ser::Error> {
        // Serialize struct to TOML value
        let value = toml::Value::try_from(&self)?;

        // Extract schema metadata (comments, field info)
        let schema = schemars::schema_for!(Self);
        let schema_info = schema::extract_schema_info(&schema, "");

        // Format TOML with comments from schema
        let result = format::format_with_comments(
            &value,
            &schema_info.comments,
            &schema_info.all_fields,
            &schema_info.optional_fields,
            "",
        );

        // Rule 14: Always end file with a single newline
        Ok(format!("{}\n", result.trim_end()))
    }
}
