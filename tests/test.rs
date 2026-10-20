extern crate dprint_development;
extern crate dprint_plugin_kdl;

use std::path::PathBuf;
use std::sync::Arc;

use dprint_core::configuration::*;
use dprint_development::*;
use dprint_plugin_kdl::configuration::Configuration;
use dprint_plugin_kdl::configuration::KdlVersion;
use dprint_plugin_kdl::configuration::resolve_config;
use dprint_plugin_kdl::*;

#[test]
fn test_specs() {
  let global_config = GlobalConfiguration::default();

  run_specs(
    &PathBuf::from("./tests/specs"),
    &ParseSpecOptions {
      default_file_name: "file.kdl",
    },
    &RunSpecsOptions {
      fix_failures: false,
      format_twice: true,
    },
    {
      let global_config = global_config.clone();
      Arc::new(move |file_path, file_text, _range, spec_config| {
        let spec_config: ConfigKeyMap = serde_json::from_value(spec_config.clone().into()).unwrap();
        let config_result = resolve_config(spec_config, &global_config);
        ensure_no_diagnostics(&config_result.diagnostics);

        format_text(file_path, file_text, &config_result.config).map_err(|err| err.into())
      })
    },
    Arc::new(move |_file_path, _file_text, _spec_config| panic!("Plugin does not support dprint-core tracing.")),
  )
}

#[test]
fn should_have_schema_defaults_matching_resolved_config() {
  let schema: serde_json::Value = serde_json::from_str(include_str!("../deployment/schema.json")).unwrap();
  let properties = schema["properties"].as_object().unwrap();
  let config = serde_json::to_value(default_config()).unwrap();
  let config = config.as_object().unwrap();
  assert_eq!(properties.len(), config.len());
  for (key, value) in config {
    assert_eq!(&properties[key]["default"], value, "default of {key}");
  }
}

#[test]
fn should_resolve_kdl_version() {
  for (value, expected) in [("v1", KdlVersion::V1), ("v2", KdlVersion::V2)] {
    let mut config = ConfigKeyMap::new();
    config.insert("kdlVersion".to_string(), ConfigKeyValue::String(value.to_string()));
    let result = resolve_config(config, &GlobalConfiguration::default());
    assert!(result.diagnostics.is_empty());
    assert_eq!(result.config.kdl_version, expected);
  }

  let mut config = ConfigKeyMap::new();
  config.insert("kdlVersion".to_string(), ConfigKeyValue::String("v3".to_string()));
  let result = resolve_config(config, &GlobalConfiguration::default());
  assert_eq!(result.diagnostics.len(), 1);
  assert_eq!(result.diagnostics[0].property_name, "kdlVersion");
}

#[test]
fn should_fail_on_v1_syntax_when_formatting_as_v2() {
  // boolean values without the `#` prefix are only valid in KDL v1.
  // See https://github.com/kachick/dprint-plugin-kdl/issues/225
  let file_path = PathBuf::from("./config.kdl");
  assert!(format_text(&file_path, "simplified_ui true\n", &default_config()).is_err());
  let v1_config = Configuration {
    kdl_version: KdlVersion::V1,
  };
  assert!(format_text(&file_path, "simplified_ui true\n", &v1_config).is_ok());
}

#[test]
fn should_fail_on_unsupported_version_marker() {
  let err = format_text(
    &PathBuf::from("./file.kdl"),
    "/- kdl-version 3\nnode \"foo\"\n",
    &default_config(),
  )
  .unwrap_err();
  assert_eq!(
    err.to_string(),
    "Unsupported KDL version '3' in version marker. Supported versions are '1' and '2'."
  );
}

fn default_config() -> Configuration {
  resolve_config(Default::default(), &GlobalConfiguration::default()).config
}
