use dprint_core::configuration::{
    ConfigKeyMap, GlobalConfiguration, get_unknown_property_diagnostics,
};
use dprint_core::plugins::{
    FileMatchingInfo, FormatError, FormatResult, PluginInfo, PluginResolveConfigurationResult,
    SyncFormatRequest, SyncHostFormatRequest, SyncPluginHandler,
};
use kdl::KdlDocument;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Configuration {}

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
        config: ConfigKeyMap,
        _global_config: &GlobalConfiguration,
    ) -> PluginResolveConfigurationResult<Configuration> {
        let mut diagnostics = Vec::new();
        diagnostics.extend(get_unknown_property_diagnostics(config));

        PluginResolveConfigurationResult {
            config: Configuration {},
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

        let text = match std::str::from_utf8(&request.file_bytes) {
            Ok(text) => text,
            Err(err) => return Err(FormatError::new(err.to_string())),
        };

        let mut doc: KdlDocument = text
            .parse::<KdlDocument>()
            .map_err(|err| FormatError::new(err.to_string()))?;
        doc.autoformat();

        let result = doc.to_string();
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
}
