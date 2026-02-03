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
pub fn extract_all_comments(schema: &RootSchema, prefix: &str) -> SchemaInfo {
    let mut comments = HashMap::new();
    let mut all_fields = HashSet::new();
    let mut optional_fields = HashSet::new();

    if let Some(obj) = schema.schema.object.as_ref() {
        for (key, sub_schema) in &obj.properties {
            let path = if prefix.is_empty() {
                key.clone()
            } else {
                format!("{}.{}", prefix, key)
            };

            all_fields.insert(path.clone());

            // Check if field is optional
            if !obj.required.contains(key) {
                optional_fields.insert(path.clone());
            }

            if let Some(metadata) = sub_schema.clone().into_object().metadata {
                if let Some(desc) = metadata.description {
                    comments.insert(path.clone(), desc);
                }
            }

            extract_nested_comments(
                sub_schema,
                &path,
                &mut comments,
                &mut all_fields,
                &mut optional_fields,
                &schema.definitions,
            );
        }
    }

    SchemaInfo {
        comments,
        all_fields,
        optional_fields,
    }
}

/// Recursively extract comments from nested schema properties
fn extract_nested_comments(
    schema: &Schema,
    prefix: &str,
    comments: &mut HashMap<String, String>,
    all_fields: &mut HashSet<String>,
    optional_fields: &mut HashSet<String>,
    definitions: &std::collections::BTreeMap<String, Schema>,
) {
    let schema_obj = schema.clone().into_object();

    // Handle references
    if let Some(reference) = &schema_obj.reference {
        let ref_name = reference
            .strip_prefix("#/definitions/")
            .unwrap_or(reference);
        if let Some(ref_schema) = definitions.get(ref_name) {
            extract_nested_comments(
                ref_schema,
                prefix,
                comments,
                all_fields,
                optional_fields,
                definitions,
            );
        }
    }

    // Handle subschemas (allOf, anyOf, oneOf)
    if let Some(subschemas) = &schema_obj.subschemas {
        if let Some(all_of) = &subschemas.all_of {
            for sub_schema in all_of {
                extract_nested_comments(
                    sub_schema,
                    prefix,
                    comments,
                    all_fields,
                    optional_fields,
                    definitions,
                );
            }
        }
        if let Some(any_of) = &subschemas.any_of {
            for sub_schema in any_of {
                extract_nested_comments(
                    sub_schema,
                    prefix,
                    comments,
                    all_fields,
                    optional_fields,
                    definitions,
                );
            }
        }
        if let Some(one_of) = &subschemas.one_of {
            for sub_schema in one_of {
                extract_nested_comments(
                    sub_schema,
                    prefix,
                    comments,
                    all_fields,
                    optional_fields,
                    definitions,
                );
            }
        }
    }

    // Handle object properties
    if let Some(obj) = &schema_obj.object {
        for (key, sub_schema) in &obj.properties {
            let path = format!("{}.{}", prefix, key);

            all_fields.insert(path.clone());

            if !obj.required.contains(key) {
                optional_fields.insert(path.clone());
            }

            if let Some(metadata) = sub_schema.clone().into_object().metadata {
                if let Some(desc) = metadata.description {
                    comments.insert(path.clone(), desc);
                }
            }

            extract_nested_comments(
                sub_schema,
                &path,
                comments,
                all_fields,
                optional_fields,
                definitions,
            );
        }
    }

    // Handle array items
    if let Some(array) = &schema_obj.array {
        if let Some(items) = &array.items {
            if let schemars::schema::SingleOrVec::Single(item_schema) = items {
                extract_nested_comments(
                    item_schema,
                    prefix,
                    comments,
                    all_fields,
                    optional_fields,
                    definitions,
                );
            }
        }
    }
}
