use schemars::{schema::RootSchema, schema::Schema};
use std::collections::HashMap;

/// Extract comments from schema root
///
/// Returns a HashMap where keys are dot-separated paths (e.g., "field", "nested.field")
/// and values are the description strings from doc comments.
///
/// For arrays of structs (e.g., `Vec<T>`), comments are extracted from the item type's fields
/// and associated with the array's path, so they apply to all items in the array-of-tables.
pub fn extract_all_comments(schema: &RootSchema, prefix: &str) -> HashMap<String, String> {
    let mut comments = HashMap::new();
    if let Some(obj) = schema.schema.object.as_ref() {
        for (key, sub_schema) in &obj.properties {
            let path = if prefix.is_empty() {
                key.clone()
            } else {
                format!("{}.{}", prefix, key)
            };

            if let Some(metadata) = sub_schema.clone().into_object().metadata {
                if let Some(desc) = metadata.description {
                    comments.insert(path.clone(), desc);
                }
            }

            extract_nested_comments(sub_schema, &path, &mut comments, &schema.definitions);
        }
    }
    comments
}

/// Recursively extract comments from nested schema properties
fn extract_nested_comments(
    schema: &Schema,
    prefix: &str,
    comments: &mut HashMap<String, String>,
    definitions: &std::collections::BTreeMap<String, Schema>,
) {
    let schema_obj = schema.clone().into_object();

    // Handle references
    if let Some(reference) = &schema_obj.reference {
        let ref_name = reference
            .strip_prefix("#/definitions/")
            .unwrap_or(reference);
        if let Some(ref_schema) = definitions.get(ref_name) {
            extract_nested_comments(ref_schema, prefix, comments, definitions);
        }
    }

    // Handle subschemas (allOf, anyOf, oneOf)
    if let Some(subschemas) = &schema_obj.subschemas {
        if let Some(all_of) = &subschemas.all_of {
            for sub_schema in all_of {
                extract_nested_comments(sub_schema, prefix, comments, definitions);
            }
        }
        if let Some(any_of) = &subschemas.any_of {
            for sub_schema in any_of {
                extract_nested_comments(sub_schema, prefix, comments, definitions);
            }
        }
        if let Some(one_of) = &subschemas.one_of {
            for sub_schema in one_of {
                extract_nested_comments(sub_schema, prefix, comments, definitions);
            }
        }
    }

    // Handle object properties
    if let Some(obj) = &schema_obj.object {
        for (key, sub_schema) in &obj.properties {
            let path = format!("{}.{}", prefix, key);

            if let Some(metadata) = sub_schema.clone().into_object().metadata {
                if let Some(desc) = metadata.description {
                    comments.insert(path.clone(), desc);
                }
            }

            extract_nested_comments(sub_schema, &path, comments, definitions);
        }
    }

    // Handle array items
    if let Some(array) = &schema_obj.array {
        if let Some(items) = &array.items {
            if let schemars::schema::SingleOrVec::Single(item_schema) = items {
                extract_nested_comments(item_schema, prefix, comments, definitions);
            }
        }
    }
}
