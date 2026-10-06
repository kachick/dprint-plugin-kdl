fn main() {
    print!("{}", include_str!(concat!(env!("OUT_DIR"), "/schema.json")));
}

#[cfg(test)]
mod tests {
    // Tests here verify that the generated schema.json is a valid JSON Schema
    // and enforces the expected property constraints (defaults, enums, additionalProperties).
    //
    // We intentionally avoid depending on external E2E test fixtures (e.g. tests/*/dprint.json)
    // here. Real fixture files are naturally validated by the dprint CLI during E2E runs
    // via `dprint check`, keeping this schema generator test self-contained.
    #[test]
    fn test_generate_json_schema() {
        let schema = include_str!(concat!(env!("OUT_DIR"), "/schema.json"));
        assert!(schema.contains("https://plugins.dprint.dev/kachick/kdl/"));
        assert!(schema.contains("/schema.json"));
        assert!(schema.contains(r#""additionalProperties": false"#));
        assert!(!schema.contains(r#""title":"#));
        assert!(!schema.contains(r#""required":"#));

        let schema_value: serde_json::Value = serde_json::from_str(schema).unwrap();
        assert_eq!(schema_value["properties"]["kdlVersion"]["default"], "v2");
        let validator = jsonschema::validator_for(&schema_value).expect("valid JSON Schema");

        let valid = serde_json::json!({});
        assert!(validator.is_valid(&valid));

        let valid_v1 = serde_json::json!({ "kdlVersion": "v1" });
        assert!(validator.is_valid(&valid_v1));

        let valid_v2 = serde_json::json!({ "kdlVersion": "v2" });
        assert!(validator.is_valid(&valid_v2));

        let invalid_version = serde_json::json!({ "kdlVersion": "v3" });
        assert!(!validator.is_valid(&invalid_version));

        let invalid = serde_json::json!({ "unknownKey": "unknown" });
        assert!(!validator.is_valid(&invalid));
    }
}
