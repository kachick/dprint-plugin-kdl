use dprint_core::plugins::FileMatchingInfo;
use dprint_core::plugins::FormatResult;
use dprint_core::plugins::PluginInfo;
use dprint_core::plugins::PluginResolveConfigurationResult;
use dprint_core::plugins::SyncFormatRequest;
use dprint_core::plugins::SyncHostFormatRequest;
use kdl::KdlDocument;
use serde::Serialize;

use anyhow::Result;
use dprint_core::configuration::ConfigKeyMap;
use dprint_core::configuration::GlobalConfiguration;
#[cfg(target_arch = "wasm32")]
use dprint_core::generate_plugin_code;
use dprint_core::plugins::SyncPluginHandler;
use std::path::Path;

pub fn format_text(file_path: &Path, text: &str, config: &Configuration) -> Result<Option<String>> {
    let result = format_text_inner(file_path, text, config)?;
    if result == text {
        Ok(None)
    } else {
        Ok(Some(result))
    }
}

fn format_text_inner(_file_path: &Path, text: &str, _config: &Configuration) -> Result<String> {
    let mut doc: KdlDocument = text.parse()?;
    doc.autoformat();

    Ok(doc.to_string())
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Configuration {}

#[derive(Default)]
pub struct KdlPluginHandler;

impl SyncPluginHandler<Configuration> for KdlPluginHandler {
    fn plugin_info(&mut self) -> PluginInfo {
        PluginInfo {
            name: env!("CARGO_PKG_NAME").to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            config_key: "kdl".to_string(),
            help_url: "https://github.com/kachick/dprint-plugin-kdl".to_string(), // fill this in
            config_schema_url: "".to_string(), // leave this empty for now
            update_url: Some("https://plugins.dprint.dev/kachick/kdl/latest.json".to_string()),
        }
    }

    fn license_text(&mut self) -> String {
        std::str::from_utf8(include_bytes!("../LICENSE"))
            .unwrap()
            .into()
    }

    fn resolve_config(
        &mut self,
        _config: ConfigKeyMap,
        _global_config: &GlobalConfiguration,
    ) -> PluginResolveConfigurationResult<Configuration> {
        let diagnostics = Vec::new();

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
            return Ok(None); // not implemented
        }

        let text = String::from_utf8_lossy(&request.file_bytes);
        format_text(request.file_path, &text, request.config)
            .map(|maybe_file_text| maybe_file_text.map(|text| text.into_bytes()))
    }

    fn check_config_updates(
        &self,
        _message: dprint_core::plugins::CheckConfigUpdatesMessage,
    ) -> Result<Vec<dprint_core::plugins::ConfigChange>> {
        Ok(Vec::new())
    }
}

#[cfg(target_arch = "wasm32")]
generate_plugin_code!(KdlPluginHandler, KdlPluginHandler, Configuration);
