mod format;
mod schema;

use schemars::JsonSchema;
use serde::Serialize;
pub use toml_template_macros::TomlTemplate;

/// Trait for generating TOML templates from Rust structs
pub trait TomlTemplate: Serialize + JsonSchema {
    /// Generate a template from an actual struct instance
    ///
    /// Returns an error if serialization fails
    fn to_template(&self) -> Result<String, toml::ser::Error> {
        let schema = schemars::schema_for!(Self);
        let value = toml::Value::try_from(&self)?;
        let schema_info = schema::extract_all_comments(&schema, "");
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
