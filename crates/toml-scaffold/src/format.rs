use crate::field_path::FieldPath;
use std::collections::{HashMap, HashSet};

/// Format TOML value with comments at the appropriate paths
pub fn format_with_comments(
    value: &toml::Value,
    comments: &HashMap<FieldPath, String>,
    all_fields: &HashSet<FieldPath>,
    optional_fields: &HashSet<FieldPath>,
    path: &FieldPath,
) -> String {
    match value {
        toml::Value::Table(table) => {
            let mut result = String::new();
            let mut inline_keys = Vec::new();
            let mut nested_tables = Vec::new();
            let mut array_tables = Vec::new();

            // Rule 15: Separate scalar fields from nested tables/arrays
            for (key, val) in table {
                match val {
                    toml::Value::Table(_) => nested_tables.push(key),
                    toml::Value::Array(arr)
                        if !arr.is_empty() && matches!(arr[0], toml::Value::Table(_)) =>
                    {
                        array_tables.push(key);
                    }
                    _ => inline_keys.push(key),
                }
            }

            // Process scalar fields first
            for key in inline_keys {
                let val = &table[key];
                let current_path = path.child(key.clone());
                append_comment(&mut result, comments, &current_path);
                // Rule 11: Use spaces around = for assignments
                result.push_str(&format!("{} = {}\n", key, format_value(val, comments)));
            }

            // Rule 17: Show missing optional fields as comments
            for field in all_fields {
                if field.len() != path.len() + 1 {
                    continue; // Not a direct child
                }
                if !field.starts_with(path) {
                    continue; // Not under current path
                }
                let key = field.get(path.len()).unwrap();

                if optional_fields.contains(field) && !table.contains_key(key) {
                    append_comment(&mut result, comments, field);
                    result.push_str(&format!("# {} = ...\n", key));
                }
            }

            // Process nested tables
            for key in nested_tables {
                let val = &table[key];
                let current_path = path.child(key.clone());
                append_section_separator(&mut result);
                append_comment(&mut result, comments, &current_path);
                // Rule 3: Always use [section] headers, never dotted keys
                result.push_str(&format!("[{}]\n", current_path.as_dotted_key()));
                result.push_str(&format_with_comments(
                    val,
                    comments,
                    all_fields,
                    optional_fields,
                    &current_path,
                ));
            }

            // Rule 5: Process array of tables using [[item]] syntax
            for key in array_tables {
                let val = &table[key];
                let current_path = path.child(key.clone());

                if let toml::Value::Array(arr) = val {
                    for item in arr {
                        append_section_separator(&mut result);
                        append_comment(&mut result, comments, &current_path);
                        result.push_str(&format!("[[{}]]\n", current_path.as_dotted_key()));
                        result.push_str(&format_with_comments(
                            item,
                            comments,
                            all_fields,
                            optional_fields,
                            &current_path,
                        ));
                    }
                }
            }

            result
        }
        _ => String::new(),
    }
}

/// Rule 7 & 9: Append comment lines above a key/section
fn append_comment(result: &mut String, comments: &HashMap<FieldPath, String>, path: &FieldPath) {
    if let Some(comment) = comments.get(path) {
        // Collapse multiple consecutive newlines into single newlines
        let normalized = comment.replace("\n\n", "\n");
        for line in normalized.lines() {
            if line.is_empty() {
                result.push_str("#\n");
            } else {
                result.push_str(&format!("# {}\n", line));
            }
        }
    }
}

/// Rule 8: Add empty line before section (if not first)
fn append_section_separator(result: &mut String) {
    if !result.is_empty() {
        result.push('\n');
    }
}

/// Convert TOML value to string representation
fn format_value(value: &toml::Value, comments: &HashMap<FieldPath, String>) -> String {
    match value {
        toml::Value::String(s) => {
            // Rule 20: Use multiline strings for strings containing newlines
            if s.contains('\n') {
                format!("\"\"\"{}\"\"\"", s)
            } else {
                // Rule 19: Properly escape strings
                let escaped = s
                    .replace('\\', "\\\\")
                    .replace('\r', "\\r")
                    .replace('\t', "\\t")
                    .replace('"', "\\\"");
                format!("\"{}\"", escaped)
            }
        }
        toml::Value::Integer(i) => i.to_string(),
        toml::Value::Float(f) => f.to_string(),
        toml::Value::Boolean(b) => b.to_string(),
        toml::Value::Array(arr) => {
            // Rule 4: Inline arrays for scalar types with less than 5 elements
            if arr.len() < 5 && arr.iter().all(|v| is_scalar(v)) {
                let items: Vec<String> = arr.iter().map(|v| format_value(v, comments)).collect();
                format!("[{}]", items.join(", "))
            } else if arr.iter().all(|v| is_scalar(v)) {
                // Rule 6: Multi-line arrays for 5+ scalar elements
                let items: Vec<String> = arr.iter().map(|v| format_value(v, comments)).collect();
                format!("[\n  {},\n]", items.join(",\n  "))
            } else {
                // Non-scalar arrays handled by format_with_comments
                format!(
                    "[{}]",
                    arr.iter()
                        .map(|v| format_value(v, comments))
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
        }
        toml::Value::Table(table) => {
            // Rule 1: Inline maps for small tables with only scalar values and no comments
            if table.len() < 5
                && table.values().all(|v| is_scalar(v))
                && !has_comments(table, comments)
            {
                let items: Vec<String> = table
                    .iter()
                    .map(|(k, v)| format!("{} = {}", k, format_value(v, comments)))
                    .collect();
                format!("{{ {} }}", items.join(", "))
            } else {
                // Rule 2: Block format for larger or complex tables
                toml::to_string(value).unwrap().trim().to_string()
            }
        }
        _ => toml::to_string(value).unwrap().trim().to_string(),
    }
}

/// Check if a TOML value is scalar (not a map or array)
fn is_scalar(value: &toml::Value) -> bool {
    !matches!(value, toml::Value::Table(_) | toml::Value::Array(_))
}

/// Check if any keys in the table have associated comments
fn has_comments(
    table: &toml::map::Map<String, toml::Value>,
    comments: &HashMap<FieldPath, String>,
) -> bool {
    table
        .keys()
        .any(|k| comments.contains_key(&FieldPath::from_vec(vec![k.clone()])))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_value_string() {
        let comments = HashMap::new();
        let val = toml::Value::String("test".to_string());
        assert_eq!(format_value(&val, &comments), "\"test\"");
    }

    #[test]
    fn test_format_value_string_with_escapes() {
        let comments = HashMap::new();
        let val = toml::Value::String("test\"quote".to_string());
        assert_eq!(format_value(&val, &comments), "\"test\\\"quote\"");
    }

    #[test]
    fn test_format_value_multiline_string() {
        let comments = HashMap::new();
        let val = toml::Value::String("line1\nline2".to_string());
        assert_eq!(format_value(&val, &comments), "\"\"\"line1\nline2\"\"\"");
    }

    #[test]
    fn test_format_value_integer() {
        let comments = HashMap::new();
        let val = toml::Value::Integer(42);
        assert_eq!(format_value(&val, &comments), "42");
    }

    #[test]
    fn test_format_value_float() {
        let comments = HashMap::new();
        let val = toml::Value::Float(3.14);
        assert_eq!(format_value(&val, &comments), "3.14");
    }

    #[test]
    fn test_format_value_boolean() {
        let comments = HashMap::new();
        let val = toml::Value::Boolean(true);
        assert_eq!(format_value(&val, &comments), "true");
    }

    #[test]
    fn test_format_value_inline_array() {
        let comments = HashMap::new();
        let val = toml::Value::Array(vec![
            toml::Value::Integer(1),
            toml::Value::Integer(2),
            toml::Value::Integer(3),
        ]);
        assert_eq!(format_value(&val, &comments), "[1, 2, 3]");
    }

    #[test]
    fn test_format_value_multiline_array() {
        let comments = HashMap::new();
        let val = toml::Value::Array(vec![
            toml::Value::Integer(1),
            toml::Value::Integer(2),
            toml::Value::Integer(3),
            toml::Value::Integer(4),
            toml::Value::Integer(5),
        ]);
        assert_eq!(
            format_value(&val, &comments),
            "[\n  1,\n  2,\n  3,\n  4,\n  5,\n]"
        );
    }

    #[test]
    fn test_is_scalar() {
        assert!(is_scalar(&toml::Value::Integer(1)));
        assert!(is_scalar(&toml::Value::String("test".to_string())));
        assert!(is_scalar(&toml::Value::Boolean(true)));
        assert!(!is_scalar(&toml::Value::Array(vec![])));
        assert!(!is_scalar(&toml::Value::Table(toml::map::Map::new())));
    }

    #[test]
    fn test_append_comment() {
        let mut result = String::new();
        let mut comments = HashMap::new();
        let path = FieldPath::from_vec(vec!["field".to_string()]);
        comments.insert(path.clone(), "Test comment".to_string());

        append_comment(&mut result, &comments, &path);
        assert_eq!(result, "# Test comment\n");
    }

    #[test]
    fn test_append_comment_multiline() {
        let mut result = String::new();
        let mut comments = HashMap::new();
        let path = FieldPath::from_vec(vec!["field".to_string()]);
        comments.insert(path.clone(), "Line 1\nLine 2".to_string());

        append_comment(&mut result, &comments, &path);
        assert_eq!(result, "# Line 1\n# Line 2\n");
    }

    #[test]
    fn test_append_section_separator() {
        let mut result = String::new();
        append_section_separator(&mut result);
        assert_eq!(result, "");

        result.push_str("content");
        append_section_separator(&mut result);
        assert_eq!(result, "content\n");
    }

    #[test]
    fn test_has_comments() {
        let mut table = toml::map::Map::new();
        table.insert("key".to_string(), toml::Value::Integer(1));

        let comments = HashMap::new();
        assert!(!has_comments(&table, &comments));

        let mut comments = HashMap::new();
        comments.insert(
            FieldPath::from_vec(vec!["key".to_string()]),
            "comment".to_string(),
        );
        assert!(has_comments(&table, &comments));
    }
}
