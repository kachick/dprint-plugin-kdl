fn main() {
    print!("{}", include_str!(concat!(env!("OUT_DIR"), "/schema.json")));
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_generate_json_schema() {
        let schema = include_str!(concat!(env!("OUT_DIR"), "/schema.json"));
        assert!(schema.contains("https://plugins.dprint.dev/kachick/kdl/"));
        assert!(schema.contains("/schema.json"));
        assert!(schema.contains(r#""additionalProperties": false"#));
        assert!(!schema.contains(r#""title":"#));
        assert!(!schema.contains(r#""required":"#));

        let schema_value: serde_json::Value = serde_json::from_str(schema).unwrap();
        let validator = jsonschema::validator_for(&schema_value).expect("valid JSON Schema");

        let valid = serde_json::json!({});
        assert!(validator.is_valid(&valid));

        let invalid = serde_json::json!({ "unknownKey": "unknown" });
        assert!(!validator.is_valid(&invalid));
    }
}
