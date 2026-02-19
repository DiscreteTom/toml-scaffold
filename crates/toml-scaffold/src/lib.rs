mod field_path;
mod format;
mod schema;

pub use field_path::FieldPath;
use schemars::JsonSchema;
use serde::Serialize;
pub use toml_scaffold_macros::TomlScaffold;

/// Trait for generating TOML scaffold files with comments from doc strings.
pub trait TomlScaffold: Serialize + JsonSchema {
    /// Returns format preferences for fields (field_path -> format_type)
    fn format_preferences() -> std::collections::HashMap<FieldPath, String> {
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
        schema_info.formats.extend(Self::format_preferences());

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

// Implementations for built-in types that return empty format preferences
macro_rules! impl_toml_scaffold_empty {
    ($($ty:ty),* $(,)?) => {
        $(
            impl TomlScaffold for $ty {
                fn format_preferences() -> std::collections::HashMap<FieldPath, String> {
                    std::collections::HashMap::new()
                }
            }
        )*
    };
}

// Primitives
impl_toml_scaffold_empty!(
    String, &str, bool, char, i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32,
    f64,
);

// Common types
impl_toml_scaffold_empty!(serde_json::Value, std::path::PathBuf,);

// Generic collections
impl<T: TomlScaffold> TomlScaffold for Vec<T> {
    fn format_preferences() -> std::collections::HashMap<FieldPath, String> {
        std::collections::HashMap::new()
    }
}

impl<T: TomlScaffold> TomlScaffold for Option<T> {
    fn format_preferences() -> std::collections::HashMap<FieldPath, String> {
        std::collections::HashMap::new()
    }
}

impl<K: Serialize + JsonSchema, V: TomlScaffold> TomlScaffold for std::collections::HashMap<K, V> {
    fn format_preferences() -> std::collections::HashMap<FieldPath, String> {
        std::collections::HashMap::new()
    }
}

impl<K: Serialize + JsonSchema, V: TomlScaffold> TomlScaffold for std::collections::BTreeMap<K, V> {
    fn format_preferences() -> std::collections::HashMap<FieldPath, String> {
        std::collections::HashMap::new()
    }
}

impl<T: TomlScaffold> TomlScaffold for std::collections::HashSet<T> {
    fn format_preferences() -> std::collections::HashMap<FieldPath, String> {
        std::collections::HashMap::new()
    }
}

impl<T: TomlScaffold> TomlScaffold for std::collections::BTreeSet<T> {
    fn format_preferences() -> std::collections::HashMap<FieldPath, String> {
        std::collections::HashMap::new()
    }
}

impl<T: TomlScaffold> TomlScaffold for Box<T> {
    fn format_preferences() -> std::collections::HashMap<FieldPath, String> {
        std::collections::HashMap::new()
    }
}
