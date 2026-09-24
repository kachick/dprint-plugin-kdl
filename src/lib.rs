use dprint_core::configuration::{
    ConfigKeyMap, GlobalConfiguration, get_nullable_value, get_unknown_property_diagnostics,
};
use dprint_core::plugins::{
    FileMatchingInfo, FormatError, FormatResult, PluginInfo, PluginResolveConfigurationResult,
    SyncFormatRequest, SyncHostFormatRequest, SyncPluginHandler,
};
use kdl::KdlDocument;
#[cfg(feature = "schema")]
use schemars::{JsonSchema, schema_for};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub enum KdlVersion {
    #[serde(rename = "v1")]
    V1,
    #[default]
    #[serde(rename = "v2")]
    V2,
}

impl std::str::FromStr for KdlVersion {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "v1" => Ok(Self::V1),
            "v2" => Ok(Self::V2),
            _ => Err(format!("Expected 'v1' or 'v2', but found '{s}'")),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct Configuration {
    #[serde(default)]
    pub kdl_version: KdlVersion,
}

#[cfg(feature = "schema")]
#[must_use]
pub fn generate_json_schema() -> String {
    let mut schema = serde_json::to_value(schema_for!(Configuration)).unwrap();
    let version = env!("CARGO_PKG_VERSION");
    if let Some(obj) = schema.as_object_mut() {
        obj.remove("title");
        obj.remove("required");
        obj.insert(
            "$id".to_string(),
            serde_json::Value::String(format!(
                "https://plugins.dprint.dev/kachick/kdl/{version}/schema.json"
            )),
        );
        obj.insert(
            "additionalProperties".to_string(),
            serde_json::Value::Bool(false),
        );

        if let Some(properties) = obj.get_mut("properties").and_then(|p| p.as_object_mut()) {
            if let Ok(serde_json::Value::Object(defaults)) =
                serde_json::to_value(Configuration::default())
            {
                for (key, default_val) in defaults {
                    if let Some(prop) = properties.get_mut(&key).and_then(|p| p.as_object_mut()) {
                        prop.insert("default".to_string(), default_val);
                    }
                }
            }
        }
    }
    serde_json::to_string_pretty(&schema).unwrap()
}

#[derive(Default)]
pub struct KdlPluginHandler;

impl SyncPluginHandler<Configuration> for KdlPluginHandler {
    fn plugin_info(&mut self) -> PluginInfo {
        let version = env!("CARGO_PKG_VERSION").to_string();
        PluginInfo {
            name: env!("CARGO_PKG_NAME").to_string(),
            version: version.clone(),
            config_key: "kdl".to_string(),
            help_url: "https://github.com/kachick/dprint-plugin-kdl".to_string(),
            config_schema_url: format!(
                "https://plugins.dprint.dev/kachick/kdl/{version}/schema.json"
            ),
            update_url: Some("https://plugins.dprint.dev/kachick/kdl/latest.json".to_string()),
        }
    }

    fn license_text(&mut self) -> String {
        include_str!("../LICENSE").to_string()
    }

    fn resolve_config(
        &mut self,
        mut config: ConfigKeyMap,
        _global_config: &GlobalConfiguration,
    ) -> PluginResolveConfigurationResult<Configuration> {
        let mut diagnostics = Vec::new();
        let kdl_version =
            get_nullable_value(&mut config, "kdlVersion", &mut diagnostics).unwrap_or_default();
        diagnostics.extend(get_unknown_property_diagnostics(config));

        PluginResolveConfigurationResult {
            config: Configuration { kdl_version },
            diagnostics,
            file_matching: FileMatchingInfo {
                file_extensions: vec!["kdl".to_string()],
                file_names: vec![],
            },
        }
    }

    fn format(
        &mut self,
        request: SyncFormatRequest<Configuration>,
        _format_with_host: impl FnMut(SyncHostFormatRequest) -> FormatResult,
    ) -> FormatResult {
        if request.range.is_some() {
            return Ok(None);
        }

        let text = std::str::from_utf8(&request.file_bytes)?;

        let result = match request.config.kdl_version {
            KdlVersion::V1 => {
                let mut doc: kdl_v1::KdlDocument = text
                    .parse::<kdl_v1::KdlDocument>()
                    .map_err(|err| FormatError::new(err.to_string()))?;
                doc.fmt();
                doc.to_string()
            }
            KdlVersion::V2 => {
                let mut doc: KdlDocument = text
                    .parse::<KdlDocument>()
                    .map_err(|err| FormatError::new(err.to_string()))?;
                doc.autoformat();
                doc.to_string()
            }
        };

        if result != text {
            Ok(Some(result.into_bytes()))
        } else {
            Ok(None)
        }
    }

    fn check_config_updates(
        &self,
        _message: dprint_core::plugins::CheckConfigUpdatesMessage,
    ) -> Result<Vec<dprint_core::plugins::ConfigChange>, FormatError> {
        Ok(Vec::new())
    }
}

#[cfg(target_arch = "wasm32")]
use dprint_core::generate_plugin_code;

// generate_plugin_code! initializes a static variable, so the second argument must be a const expression.
// Default::default() cannot be used here because trait methods cannot be called in statics.
#[cfg(target_arch = "wasm32")]
generate_plugin_code!(KdlPluginHandler, KdlPluginHandler);

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use dprint_core::configuration::ConfigKeyValue;
    use dprint_core::plugins::{FormatConfigId, NullCancellationToken};

    use super::*;

    #[test]
    fn test_resolve_config_defaults() {
        let mut handler = KdlPluginHandler;
        let result = handler.resolve_config(ConfigKeyMap::new(), &GlobalConfiguration::default());
        assert!(result.diagnostics.is_empty());
        assert_eq!(result.config, Configuration::default());
        assert_eq!(result.file_matching.file_extensions, vec!["kdl"]);
    }

    #[test]
    fn test_resolve_config_unknown_property() {
        let mut handler = KdlPluginHandler;
        let mut config = ConfigKeyMap::new();
        config.insert(
            "unknownProp".to_string(),
            ConfigKeyValue::String("val".to_string()),
        );
        let result = handler.resolve_config(config, &GlobalConfiguration::default());
        assert_eq!(result.diagnostics.len(), 1);
        assert_eq!(result.diagnostics[0].property_name, "unknownProp");
    }

    #[test]
    fn test_format() {
        let mut handler = KdlPluginHandler;
        let resolve_result =
            handler.resolve_config(ConfigKeyMap::new(), &GlobalConfiguration::default());
        let cancellation_token = NullCancellationToken;
        let request = SyncFormatRequest {
            file_path: &PathBuf::from("test.kdl"),
            file_bytes: b"node   key=\"value\"".to_vec(),
            config_id: FormatConfigId::from_raw(1),
            config: &resolve_result.config,
            range: None,
            token: &cancellation_token,
        };
        let formatted = handler.format(request, |_| unreachable!()).unwrap();
        assert!(formatted.is_some());
        let formatted_str = String::from_utf8(formatted.unwrap()).unwrap();
        assert_eq!(formatted_str, "node key=value\n");
    }

    #[test]
    fn test_format_unchanged_returns_none() {
        let mut handler = KdlPluginHandler;
        let resolve_result =
            handler.resolve_config(ConfigKeyMap::new(), &GlobalConfiguration::default());
        let cancellation_token = NullCancellationToken;
        let request = SyncFormatRequest {
            file_path: &PathBuf::from("test.kdl"),
            file_bytes: b"node key=value\n".to_vec(),
            config_id: FormatConfigId::from_raw(1),
            config: &resolve_result.config,
            range: None,
            token: &cancellation_token,
        };
        let formatted = handler.format(request, |_| unreachable!()).unwrap();
        assert_eq!(formatted, None);
    }

    #[test]
    fn test_format_range_returns_none() {
        let mut handler = KdlPluginHandler;
        let resolve_result =
            handler.resolve_config(ConfigKeyMap::new(), &GlobalConfiguration::default());
        let cancellation_token = NullCancellationToken;
        let request = SyncFormatRequest {
            file_path: &PathBuf::from("test.kdl"),
            file_bytes: b"node key=\"value\"\n".to_vec(),
            config_id: FormatConfigId::from_raw(1),
            config: &resolve_result.config,
            range: Some(std::ops::Range { start: 0, end: 5 }),
            token: &cancellation_token,
        };
        let formatted = handler.format(request, |_| unreachable!()).unwrap();
        assert_eq!(formatted, None);
    }

    #[test]
    fn test_format_invalid_utf8() {
        let mut handler = KdlPluginHandler;
        let resolve_result =
            handler.resolve_config(ConfigKeyMap::new(), &GlobalConfiguration::default());
        let cancellation_token = NullCancellationToken;
        let request = SyncFormatRequest {
            file_path: &PathBuf::from("test.kdl"),
            file_bytes: vec![0xFF, 0xFE, 0xFD],
            config_id: FormatConfigId::from_raw(1),
            config: &resolve_result.config,
            range: None,
            token: &cancellation_token,
        };
        let result = handler.format(request, |_| unreachable!());
        assert!(result.is_err());
    }

    #[test]
    fn test_resolve_config_kdl_version() {
        let mut handler = KdlPluginHandler;

        // Explicit v1
        let mut config_v1 = ConfigKeyMap::new();
        config_v1.insert(
            "kdlVersion".to_string(),
            ConfigKeyValue::String("v1".to_string()),
        );
        let result_v1 = handler.resolve_config(config_v1, &GlobalConfiguration::default());
        assert_eq!(result_v1.config.kdl_version, KdlVersion::V1);
        assert!(result_v1.diagnostics.is_empty());

        // Explicit v2
        let mut config_v2 = ConfigKeyMap::new();
        config_v2.insert(
            "kdlVersion".to_string(),
            ConfigKeyValue::String("v2".to_string()),
        );
        let result_v2 = handler.resolve_config(config_v2, &GlobalConfiguration::default());
        assert_eq!(result_v2.config.kdl_version, KdlVersion::V2);
        assert!(result_v2.diagnostics.is_empty());

        // Invalid version
        let mut config_invalid = ConfigKeyMap::new();
        config_invalid.insert(
            "kdlVersion".to_string(),
            ConfigKeyValue::String("v3".to_string()),
        );
        let result_invalid =
            handler.resolve_config(config_invalid, &GlobalConfiguration::default());
        assert_eq!(result_invalid.diagnostics.len(), 1);
        assert_eq!(result_invalid.diagnostics[0].property_name, "kdlVersion");
    }

    #[test]
    fn test_format_v1_and_v2_differences() {
        let mut handler = KdlPluginHandler;
        let cancellation_token = NullCancellationToken;

        // In KDL v1:
        // - `key="value"` preserves quotes for string values
        // - `flag=true` is valid boolean syntax
        let v1_input = b"node   key=\"value\"   flag=true".to_vec();
        let config_v1 = Configuration {
            kdl_version: KdlVersion::V1,
        };
        let request_v1 = SyncFormatRequest {
            file_path: &PathBuf::from("test.kdl"),
            file_bytes: v1_input,
            config_id: FormatConfigId::from_raw(1),
            config: &config_v1,
            range: None,
            token: &cancellation_token,
        };
        let formatted_v1 = handler.format(request_v1, |_| unreachable!()).unwrap();
        assert!(formatted_v1.is_some());
        let formatted_str_v1 = String::from_utf8(formatted_v1.unwrap()).unwrap();
        assert_eq!(formatted_str_v1, "node key=\"value\" flag=true\n");

        // In KDL v2:
        // - `key="value"` strips quotes for identifier-like strings -> `key=value`
        // - boolean requires `#` prefix -> `flag=#true`
        let v2_input = b"node   key=\"value\"   flag=#true".to_vec();
        let config_v2 = Configuration {
            kdl_version: KdlVersion::V2,
        };
        let request_v2 = SyncFormatRequest {
            file_path: &PathBuf::from("test.kdl"),
            file_bytes: v2_input,
            config_id: FormatConfigId::from_raw(2),
            config: &config_v2,
            range: None,
            token: &cancellation_token,
        };
        let formatted_v2 = handler.format(request_v2, |_| unreachable!()).unwrap();
        assert!(formatted_v2.is_some());
        let formatted_str_v2 = String::from_utf8(formatted_v2.unwrap()).unwrap();
        assert_eq!(formatted_str_v2, "node key=value flag=#true\n");
    }
}
