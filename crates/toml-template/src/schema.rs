use schemars::{schema::RootSchema, schema::Schema};
use std::collections::{HashMap, HashSet};

/// Schema information extracted from a JSON schema
pub struct SchemaInfo {
    /// Doc comments for fields. Keys are dot-separated paths (e.g., "field", "nested.field")
    pub comments: HashMap<String, String>,
    /// All field paths in the schema. Keys are dot-separated paths (e.g., "field", "nested.field")
    pub all_fields: HashSet<String>,
    /// Optional field paths. Keys are dot-separated paths (e.g., "field", "nested.field")
    pub optional_fields: HashSet<String>,
}

/// Extract comments and field information from schema root
pub fn extract_schema_info(schema: &RootSchema, prefix: &str) -> SchemaInfo {
    let mut info = SchemaInfo {
        comments: HashMap::new(),
        all_fields: HashSet::new(),
        optional_fields: HashSet::new(),
    };

    if let Some(obj) = schema.schema.object.as_ref() {
        for (key, sub_schema) in &obj.properties {
            let path = if prefix.is_empty() {
                key.clone()
            } else {
                format!("{}.{}", prefix, key)
            };

            info.all_fields.insert(path.clone());

            // Check if field is optional
            if !obj.required.contains(key) {
                info.optional_fields.insert(path.clone());
            }

            if let Some(metadata) = sub_schema.clone().into_object().metadata {
                if let Some(desc) = metadata.description {
                    info.comments.insert(path.clone(), desc);
                }
            }

            extract_nested_schema_info(sub_schema, &path, &mut info, &schema.definitions);
        }
    }

    info
}

/// Recursively extract comments from nested schema properties
fn extract_nested_schema_info(
    schema: &Schema,
    prefix: &str,
    info: &mut SchemaInfo,
    definitions: &std::collections::BTreeMap<String, Schema>,
) {
    let schema_obj = schema.clone().into_object();

    // Handle references
    if let Some(reference) = &schema_obj.reference {
        let ref_name = reference
            .strip_prefix("#/definitions/")
            .unwrap_or(reference);
        if let Some(ref_schema) = definitions.get(ref_name) {
            extract_nested_schema_info(ref_schema, prefix, info, definitions);
        }
    }

    // Handle subschemas (allOf, anyOf, oneOf)
    if let Some(subschemas) = &schema_obj.subschemas {
        if let Some(all_of) = &subschemas.all_of {
            for sub_schema in all_of {
                extract_nested_schema_info(sub_schema, prefix, info, definitions);
            }
        }
        if let Some(any_of) = &subschemas.any_of {
            for sub_schema in any_of {
                extract_nested_schema_info(sub_schema, prefix, info, definitions);
            }
        }
        if let Some(one_of) = &subschemas.one_of {
            for sub_schema in one_of {
                extract_nested_schema_info(sub_schema, prefix, info, definitions);
            }
        }
    }

    // Handle object properties
    if let Some(obj) = &schema_obj.object {
        for (key, sub_schema) in &obj.properties {
            let path = format!("{}.{}", prefix, key);

            info.all_fields.insert(path.clone());

            if !obj.required.contains(key) {
                info.optional_fields.insert(path.clone());
            }

            if let Some(metadata) = sub_schema.clone().into_object().metadata {
                if let Some(desc) = metadata.description {
                    info.comments.insert(path.clone(), desc);
                }
            }

            extract_nested_schema_info(sub_schema, &path, info, definitions);
        }
    }

    // Handle array items
    if let Some(array) = &schema_obj.array {
        if let Some(items) = &array.items {
            if let schemars::schema::SingleOrVec::Single(item_schema) = items {
                extract_nested_schema_info(item_schema, prefix, info, definitions);
            }
        }
    }
}
