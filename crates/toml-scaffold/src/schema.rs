use schemars::Schema;
use serde_json::Value;
use std::collections::{HashMap, HashSet};

use crate::field_path::FieldPath;

/// Schema information extracted from a JSON schema.
pub struct SchemaInfo {
    /// Doc comments for fields
    pub comments: HashMap<FieldPath, String>,
    /// All field paths in the schema
    pub all_fields: HashSet<FieldPath>,
    /// Optional field paths
    pub optional_fields: HashSet<FieldPath>,
}

/// Extracts comments and field information from schema root.
pub fn extract_schema_info(schema: &Schema, prefix: &FieldPath) -> SchemaInfo {
    let mut info = SchemaInfo {
        comments: HashMap::new(),
        all_fields: HashSet::new(),
        optional_fields: HashSet::new(),
    };

    let Some(obj) = schema.as_object() else {
        return info;
    };

    let definitions = obj
        .get("$defs")
        .and_then(|v| v.as_object())
        .cloned()
        .unwrap_or_default();

    process_properties(obj, prefix, &mut info, &definitions);
    info
}

/// Recursively extract comments from nested schema properties
fn extract_nested_schema_info(
    schema: &Value,
    prefix: &FieldPath,
    info: &mut SchemaInfo,
    definitions: &serde_json::Map<String, Value>,
) {
    let Some(obj) = schema.as_object() else {
        return;
    };

    // Handle references
    if let Some(reference) = obj.get("$ref").and_then(|v| v.as_str()) {
        let ref_name = reference.strip_prefix("#/$defs/").unwrap_or(reference);
        if let Some(ref_schema) = definitions.get(ref_name) {
            extract_nested_schema_info(ref_schema, prefix, info, definitions);
        }
    }

    // Handle subschemas (allOf, anyOf, oneOf)
    for key in ["allOf", "anyOf", "oneOf"] {
        if let Some(subschemas) = obj.get(key).and_then(|v| v.as_array()) {
            for sub_schema in subschemas {
                extract_nested_schema_info(sub_schema, prefix, info, definitions);
            }
        }
    }

    process_properties(obj, prefix, info, definitions);

    // Handle array items
    if let Some(items) = obj.get("items") {
        extract_nested_schema_info(items, prefix, info, definitions);
    }
}

/// Process properties from a schema object
fn process_properties(
    obj: &serde_json::Map<String, Value>,
    prefix: &FieldPath,
    info: &mut SchemaInfo,
    definitions: &serde_json::Map<String, Value>,
) {
    let Some(properties) = obj.get("properties").and_then(|v| v.as_object()) else {
        return;
    };

    let required = obj
        .get("required")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str())
                .collect::<HashSet<_>>()
        })
        .unwrap_or_default();

    for (key, sub_schema) in properties {
        let path = prefix.child(key.clone());

        info.all_fields.insert(path.clone());

        if !required.contains(key.as_str()) {
            info.optional_fields.insert(path.clone());
        }

        if let Some(desc) = sub_schema.get("description").and_then(|v| v.as_str()) {
            info.comments.insert(path.clone(), desc.to_string());
        }

        extract_nested_schema_info(sub_schema, &path, info, definitions);
    }
}
