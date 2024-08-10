use dprint_core::configuration::ResolveConfigurationResult;
// use dprint_core::formatting::PrintOptions;
use dprint_core::plugins::FileMatchingInfo;
use dprint_core::plugins::FormatResult;
use dprint_core::plugins::SyncPluginInfo;
use serde::Serialize;

use anyhow::Result;
use dprint_core::configuration::ConfigKeyMap;
use dprint_core::configuration::GlobalConfiguration;
#[cfg(target_arch = "wasm32")]
use dprint_core::generate_plugin_code;
use dprint_core::plugins::PluginInfo;
use dprint_core::plugins::SyncPluginHandler;
use std::path::Path;

#[inline]
pub fn parse_kdl(input: &str) -> Result<kdl::KdlDocument, kdl::KdlError> {
    input.parse::<kdl::KdlDocument>()
}

#[inline]
pub fn format_kdl(mut input: kdl::KdlDocument) -> String {
    // https://github.com/kdl-org/kdl-rs/blob/6044ef9776f24f45004c36d7628b1f5fbd83c8ad/src/entry.rs#L193-L212
    input.fmt();

    input.to_string()
}

pub fn format_text(file_path: &Path, text: &str, config: &Configuration) -> Result<Option<String>> {
    let result = format_text_inner(file_path, text, config)?;
    if result == text {
        Ok(None)
    } else {
        Ok(Some(result))
    }
}

fn format_text_inner(_file_path: &Path, text: &str, _config: &Configuration) -> Result<String> {
    let text = strip_bom(text);

    let parsed = parse_kdl(&text)?;

    let formatted = format_kdl(parsed);

    Ok(formatted)
}

fn strip_bom(text: &str) -> &str {
    text.strip_prefix("\u{FEFF}").unwrap_or(text)
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Configuration {}

#[derive(Default)]
pub struct KdlPluginHandler;

impl SyncPluginHandler<Configuration> for KdlPluginHandler {
    fn plugin_info(&mut self) -> SyncPluginInfo {
        SyncPluginInfo {
            info: PluginInfo {
                name: env!("CARGO_PKG_NAME").to_string(),
                version: env!("CARGO_PKG_VERSION").to_string(),
                config_key: "kdl".to_string(),
                help_url: "https://github.com/kachick/dprint-plugin-kdl".to_string(), // fill this in
                config_schema_url: "".to_string(), // leave this empty for now
                update_url: None,                  // leave this empty for now
            },
            file_matching: FileMatchingInfo {
                file_extensions: vec!["kdl".to_string()],
                file_names: vec![],
            },
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
    ) -> ResolveConfigurationResult<Configuration> {
        let diagnostics = Vec::new();

        ResolveConfigurationResult {
            config: Configuration {},
            diagnostics,
        }
    }

    fn format(
        &mut self,
        file_path: &Path,
        file_bytes: Vec<u8>,
        config: &Configuration,
        mut _format_with_host: impl FnMut(&Path, Vec<u8>, &ConfigKeyMap) -> FormatResult,
    ) -> FormatResult {
        let file_text = String::from_utf8(file_bytes)?;
        format_text(file_path, &file_text, config)
            .map(|maybe_file_text| maybe_file_text.map(|text| text.into_bytes()))
    }
}

#[cfg(target_arch = "wasm32")]
generate_plugin_code!(KdlPluginHandler, KdlPluginHandler);
