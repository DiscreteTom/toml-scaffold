/// A field path represented as a sequence of field names.
///
/// This struct is used to represent paths to fields in nested structures,
/// where each segment is a field name.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FieldPath(Vec<String>);

impl FieldPath {
    /// Creates an empty field path.
    pub fn new() -> Self {
        Self(Vec::new())
    }

    /// Creates a field path from a vector of segments.
    pub fn from_vec(segments: Vec<String>) -> Self {
        Self(segments)
    }

    /// Appends a segment to the end of this path.
    pub fn push(&mut self, segment: String) {
        self.0.push(segment);
    }

    /// Creates a new path by appending a segment to this path.
    pub fn child(&self, segment: String) -> Self {
        let mut path = self.clone();
        path.push(segment);
        path
    }

    /// Returns the number of segments in this path.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Checks if this path starts with another path.
    pub fn starts_with(&self, other: &FieldPath) -> bool {
        self.0.starts_with(&other.0)
    }

    /// Returns the segment at the given index.
    pub fn get(&self, index: usize) -> Option<&String> {
        self.0.get(index)
    }

    /// Converts this path to a TOML dotted key string.
    pub fn as_dotted_key(&self) -> String {
        self.0.join(".")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_new() {
        let path = FieldPath::new();
        assert_eq!(path.len(), 0);
        assert_eq!(path.as_dotted_key(), "");
    }

    #[test]
    fn test_from_vec() {
        let path = FieldPath::from_vec(vec!["a".to_string(), "b".to_string()]);
        assert_eq!(path.len(), 2);
        assert_eq!(path.as_dotted_key(), "a.b");
    }

    #[test]
    fn test_push() {
        let mut path = FieldPath::new();
        path.push("first".to_string());
        path.push("second".to_string());
        assert_eq!(path.len(), 2);
        assert_eq!(path.as_dotted_key(), "first.second");
    }

    #[test]
    fn test_child() {
        let parent = FieldPath::from_vec(vec!["parent".to_string()]);
        let child = parent.child("child".to_string());
        assert_eq!(parent.len(), 1);
        assert_eq!(child.len(), 2);
        assert_eq!(child.as_dotted_key(), "parent.child");
    }

    #[test]
    fn test_starts_with() {
        let parent = FieldPath::from_vec(vec!["a".to_string(), "b".to_string()]);
        let child = FieldPath::from_vec(vec!["a".to_string(), "b".to_string(), "c".to_string()]);
        let other = FieldPath::from_vec(vec!["x".to_string()]);

        assert!(child.starts_with(&parent));
        assert!(!child.starts_with(&other));
        assert!(parent.starts_with(&parent));
    }

    #[test]
    fn test_get() {
        let path = FieldPath::from_vec(vec!["a".to_string(), "b".to_string(), "c".to_string()]);
        assert_eq!(path.get(0), Some(&"a".to_string()));
        assert_eq!(path.get(1), Some(&"b".to_string()));
        assert_eq!(path.get(2), Some(&"c".to_string()));
        assert_eq!(path.get(3), None);
    }

    #[test]
    fn test_field_with_dots() {
        let path = FieldPath::from_vec(vec!["field.with.dots".to_string()]);
        assert_eq!(path.len(), 1);
        assert_eq!(path.as_dotted_key(), "field.with.dots");
    }

    #[test]
    fn test_hash_map_key() {
        let mut map = HashMap::new();
        let path1 = FieldPath::from_vec(vec!["a".to_string(), "b".to_string()]);
        let path2 = FieldPath::from_vec(vec!["a".to_string(), "b".to_string()]);
        let path3 = FieldPath::from_vec(vec!["a".to_string(), "c".to_string()]);

        map.insert(path1.clone(), "value1");
        map.insert(path3, "value2");

        assert_eq!(map.get(&path2), Some(&"value1"));
        assert_eq!(map.len(), 2);
    }
}
