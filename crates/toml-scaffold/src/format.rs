use crate::field_path::FieldPath;
use std::collections::{HashMap, HashSet};
use toml_writer::TomlWrite;

/// Format TOML value with comments at the appropriate paths
pub fn format_with_comments(
    value: &toml::Value,
    comments: &HashMap<FieldPath, String>,
    all_fields: &HashSet<FieldPath>,
    optional_fields: &HashSet<FieldPath>,
    formats: &HashMap<FieldPath, String>,
    path: &FieldPath,
) -> String {
    match value {
        toml::Value::Table(table) => {
            let mut result = String::new();

            // Append root description if at root level
            if path.len() == 0 {
                append_comment(&mut result, comments, &FieldPath::new());
                if comments.contains_key(&FieldPath::new()) {
                    result.push('\n');
                }
            }

            let (inline_keys, nested_tables, array_tables) = categorize_table_keys(table);

            // Process scalar fields first
            for key in inline_keys {
                let val = &table[key];
                let current_path = path.child(key.clone());
                append_comment(&mut result, comments, &current_path);
                // Rule 11: Use spaces around = for assignments
                let _ = result.key(key.as_str());
                result.push_str(&format!(
                    " = {}\n",
                    format_value(val, comments, formats, &current_path)
                ));
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
                    result.push_str("# ");
                    let _ = result.key(key.as_str());
                    result.push_str(" = ...\n");
                }
            }

            // Process nested tables
            for key in nested_tables {
                let val = &table[key];
                let current_path = path.child(key.clone());
                let format_pref = formats.get(&current_path).map(|s| s.as_str());

                match format_pref {
                    Some("inline") => {
                        // Inline format: key = { ... }
                        append_comment(&mut result, comments, &current_path);
                        let _ = result.key(key.as_str());
                        if let toml::Value::Table(nested) = val {
                            let items: Vec<String> = nested
                                .iter()
                                .map(|(k, v)| {
                                    let subpath = current_path.child(k.clone());
                                    format!(
                                        "{} = {}",
                                        k,
                                        format_value(v, comments, formats, &subpath)
                                    )
                                })
                                .collect();
                            result.push_str(&format!(" = {{ {} }}\n", items.join(", ")));
                        }
                    }
                    Some("dotted") => {
                        // Dotted format: key.subkey = value (one level only)
                        append_comment(&mut result, comments, &current_path);
                        if let toml::Value::Table(nested) = val {
                            for (subkey, subval) in nested {
                                let subpath = current_path.child(subkey.clone());
                                append_comment(&mut result, comments, &subpath);
                                result.push_str(&format!(
                                    "{}.{} = {}\n",
                                    key,
                                    subkey,
                                    format_value(subval, comments, formats, &subpath)
                                ));
                            }
                        }
                    }
                    Some("dotted-nested") => {
                        // Dotted nested format: recursively flatten all levels
                        append_comment(&mut result, comments, &current_path);
                        if let toml::Value::Table(nested) = val {
                            flatten_dotted(
                                &mut result,
                                key,
                                nested,
                                comments,
                                formats,
                                &current_path,
                            );
                        }
                    }
                    Some(fmt) if fmt.starts_with("*") => {
                        // Child format: [section] with custom format for children
                        let child_format = &fmt[1..]; // Remove * prefix
                        append_section_separator(&mut result);
                        append_comment(&mut result, comments, &current_path);
                        result.push_str(&format!("[{}]\n", current_path.as_dotted_key()));

                        if let toml::Value::Table(nested) = val {
                            match child_format {
                                "dotted" => {
                                    // Children use dotted format
                                    for (subkey, subval) in nested {
                                        let subpath = current_path.child(subkey.clone());
                                        if let toml::Value::Table(sub_nested) = subval {
                                            for (sub_subkey, sub_subval) in sub_nested {
                                                let sub_subpath = subpath.child(sub_subkey.clone());
                                                append_comment(&mut result, comments, &sub_subpath);
                                                result.push_str(&format!(
                                                    "{}.{} = {}\n",
                                                    subkey,
                                                    sub_subkey,
                                                    format_value(
                                                        sub_subval,
                                                        comments,
                                                        formats,
                                                        &sub_subpath
                                                    )
                                                ));
                                            }
                                        } else {
                                            append_comment(&mut result, comments, &subpath);
                                            result.push_str(&format!(
                                                "{} = {}\n",
                                                subkey,
                                                format_value(subval, comments, formats, &subpath)
                                            ));
                                        }
                                    }
                                }
                                "dotted-nested" => {
                                    // Children use dotted-nested format
                                    for (subkey, subval) in nested {
                                        let subpath = current_path.child(subkey.clone());
                                        if let toml::Value::Table(sub_nested) = subval {
                                            flatten_dotted(
                                                &mut result,
                                                subkey,
                                                sub_nested,
                                                comments,
                                                formats,
                                                &subpath,
                                            );
                                        } else {
                                            append_comment(&mut result, comments, &subpath);
                                            result.push_str(&format!(
                                                "{} = {}\n",
                                                subkey,
                                                format_value(subval, comments, formats, &subpath)
                                            ));
                                        }
                                    }
                                }
                                _ => {
                                    // Default standard format for children
                                    result.push_str(&format_with_comments(
                                        val,
                                        comments,
                                        all_fields,
                                        optional_fields,
                                        formats,
                                        &current_path,
                                    ));
                                }
                            }
                        }
                    }
                    _ => {
                        // Standard format: [section]
                        append_section_separator(&mut result);
                        append_comment(&mut result, comments, &current_path);
                        result.push_str(&format!("[{}]\n", current_path.as_dotted_key()));
                        result.push_str(&format_with_comments(
                            val,
                            comments,
                            all_fields,
                            optional_fields,
                            formats,
                            &current_path,
                        ));
                    }
                }
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
                            formats,
                            &current_path,
                        ));
                    }
                }
            }

            result
        }
        _ => unreachable!(),
    }
}

/// Rule 15: Categorize table keys into inline, nested tables, and array tables
fn categorize_table_keys(
    table: &toml::map::Map<String, toml::Value>,
) -> (Vec<&String>, Vec<&String>, Vec<&String>) {
    let mut inline_keys = Vec::new();
    let mut nested_tables = Vec::new();
    let mut array_tables = Vec::new();

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

    (inline_keys, nested_tables, array_tables)
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
    if !result.is_empty() && !result.ends_with("\n\n") {
        result.push('\n');
    }
}

/// Convert TOML value to string representation
fn format_value(
    value: &toml::Value,
    comments: &HashMap<FieldPath, String>,
    formats: &HashMap<FieldPath, String>,
    path: &FieldPath,
) -> String {
    match value {
        toml::Value::String(s) => {
            let mut result = String::new();
            let _ = result.value(s);
            result
        }
        toml::Value::Integer(i) => {
            let mut result = String::new();
            let _ = result.value(i);
            result
        }
        toml::Value::Float(f) => {
            let mut result = String::new();
            let _ = result.value(f);
            result
        }
        toml::Value::Boolean(b) => {
            let mut result = String::new();
            let _ = result.value(b);
            result
        }
        toml::Value::Array(arr) => {
            let format_pref = formats.get(path).map(|s| s.as_str());

            // Check if multiline format is requested
            if format_pref == Some("multiline") {
                let items: Vec<String> = arr
                    .iter()
                    .map(|v| format_value(v, comments, formats, path))
                    .collect();
                return format!("[\n  {},\n]", items.join(",\n  "));
            }

            // Rule 4: Inline arrays for scalar types
            if arr.iter().all(|v| is_scalar(v)) {
                let items: Vec<String> = arr
                    .iter()
                    .map(|v| format_value(v, comments, formats, path))
                    .collect();
                format!("[{}]", items.join(", "))
            } else {
                // Non-scalar arrays handled by format_with_comments
                format!(
                    "[{}]",
                    arr.iter()
                        .map(|v| format_value(v, comments, formats, path))
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
                    .map(|(k, v)| format!("{} = {}", k, format_value(v, comments, formats, path)))
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

/// Recursively flatten nested tables into dotted keys
fn flatten_dotted(
    result: &mut String,
    prefix: &str,
    table: &toml::map::Map<String, toml::Value>,
    comments: &HashMap<FieldPath, String>,
    formats: &HashMap<FieldPath, String>,
    path: &FieldPath,
) {
    for (key, val) in table {
        let subpath = path.child(key.clone());
        let dotted_key = format!("{}.{}", prefix, key);

        match val {
            toml::Value::Table(nested) => {
                // Recursively flatten nested tables
                flatten_dotted(result, &dotted_key, nested, comments, formats, &subpath);
            }
            _ => {
                // Scalar value - write as dotted key
                append_comment(result, comments, &subpath);
                result.push_str(&format!(
                    "{} = {}\n",
                    dotted_key,
                    format_value(val, comments, formats, &subpath)
                ));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_value_string() {
        let comments = HashMap::new();
        let formats = HashMap::new();
        let path = FieldPath::new();
        let val = toml::Value::String("test".to_string());
        assert_eq!(format_value(&val, &comments, &formats, &path), "\"test\"");
    }

    #[test]
    fn test_format_value_string_with_escapes() {
        let comments = HashMap::new();
        let formats = HashMap::new();
        let path = FieldPath::new();
        let val = toml::Value::String("test\"quote".to_string());
        // TomlStringBuilder uses literal strings for strings with quotes
        assert_eq!(
            format_value(&val, &comments, &formats, &path),
            "'test\"quote'"
        );
    }

    #[test]
    fn test_format_value_multiline_string() {
        let comments = HashMap::new();
        let formats = HashMap::new();
        let path = FieldPath::new();
        let val = toml::Value::String("line1\nline2".to_string());
        // TomlStringBuilder adds newline after opening """ for multiline strings
        assert_eq!(
            format_value(&val, &comments, &formats, &path),
            "\"\"\"\nline1\nline2\"\"\""
        );
    }

    #[test]
    fn test_format_value_integer() {
        let comments = HashMap::new();
        let formats = HashMap::new();
        let path = FieldPath::new();
        let val = toml::Value::Integer(42);
        assert_eq!(format_value(&val, &comments, &formats, &path), "42");
    }

    #[test]
    fn test_format_value_float() {
        let comments = HashMap::new();
        let formats = HashMap::new();
        let path = FieldPath::new();
        let val = toml::Value::Float(3.14);
        assert_eq!(format_value(&val, &comments, &formats, &path), "3.14");
    }

    #[test]
    fn test_format_value_boolean() {
        let comments = HashMap::new();
        let formats = HashMap::new();
        let path = FieldPath::new();
        let val = toml::Value::Boolean(true);
        assert_eq!(format_value(&val, &comments, &formats, &path), "true");
    }

    #[test]
    fn test_format_value_inline_array() {
        let comments = HashMap::new();
        let formats = HashMap::new();
        let path = FieldPath::new();
        let val = toml::Value::Array(vec![
            toml::Value::Integer(1),
            toml::Value::Integer(2),
            toml::Value::Integer(3),
        ]);
        assert_eq!(format_value(&val, &comments, &formats, &path), "[1, 2, 3]");
    }

    #[test]
    fn test_format_value_multiline_array() {
        let comments = HashMap::new();
        let formats = HashMap::new();
        let path = FieldPath::new();
        let val = toml::Value::Array(vec![
            toml::Value::Integer(1),
            toml::Value::Integer(2),
            toml::Value::Integer(3),
            toml::Value::Integer(4),
            toml::Value::Integer(5),
        ]);
        assert_eq!(
            format_value(&val, &comments, &formats, &path),
            "[1, 2, 3, 4, 5]"
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
