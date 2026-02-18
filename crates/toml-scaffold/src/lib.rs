mod field_path;
mod format;
mod schema;

use field_path::FieldPath;
use schemars::JsonSchema;
use serde::Serialize;
pub use toml_scaffold_macros::TomlScaffold;

/// Trait for generating TOML scaffold files with comments from doc strings.
pub trait TomlScaffold: Serialize + JsonSchema {
    /// Returns format preferences for fields (field_name -> format_type)
    fn format_preferences() -> std::collections::HashMap<String, String> {
        std::collections::HashMap::new()
    }

    /// Generates a TOML scaffold string with comments from struct field doc comments.
    fn to_scaffold(&self) -> Result<String, toml::ser::Error> {
        // Serialize struct to TOML value
        let value = toml::Value::try_from(&self)?;

        // Extract schema metadata (comments, field info)
        let schema = schemars::schema_for!(Self);
        let mut schema_info = schema::extract_schema_info(&schema, &FieldPath::new());

        // Apply format preferences
        let prefs = Self::format_preferences();
        for (field, format) in prefs {
            let path = FieldPath::from_vec(vec![field]);
            schema_info.formats.insert(path, format);
        }

        // Format TOML with comments from schema
        let result = format::format_with_comments(
            &value,
            &schema_info.comments,
            &schema_info.all_fields,
            &schema_info.optional_fields,
            &schema_info.formats,
            &FieldPath::new(),
        );

        // Rule 14: Always end file with a single newline
        Ok(format!("{}\n", result.trim_end()))
    }
}
