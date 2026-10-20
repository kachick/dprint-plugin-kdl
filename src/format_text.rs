use std::path::Path;

use crate::configuration::Configuration;
use crate::configuration::KdlVersion;

/// Error that occurred while formatting KDL text.
#[derive(Debug, thiserror::Error)]
pub enum FormatError {
  /// The version marker at the top of the file is for a version that isn't supported.
  #[error("Unsupported KDL version '{0}' in version marker. Supported versions are '1' and '2'.")]
  UnsupportedVersionMarker(String),
  /// The text could not be parsed as KDL v2.
  #[error(transparent)]
  Kdl(#[from] kdl::KdlError),
  /// The text could not be parsed as KDL v1.
  #[error(transparent)]
  KdlV1(#[from] kdl_v1::KdlError),
}

pub fn format_text(_file_path: &Path, input_text: &str, config: &Configuration) -> Result<Option<String>, FormatError> {
  let kdl_version = detect_version_marker(input_text)?.unwrap_or(config.kdl_version);
  let formatted = match kdl_version {
    KdlVersion::V1 => {
      let mut doc = input_text.parse::<kdl_v1::KdlDocument>()?;
      doc.fmt();
      doc.to_string()
    }
    KdlVersion::V2 => {
      let mut doc = input_text.parse::<kdl::KdlDocument>()?;
      doc.autoformat();
      doc.to_string()
    }
  };
  Ok(if formatted == input_text { None } else { Some(formatted) })
}

// The version marker detection logic is inspired by `kdl-rs` (src/document.rs).
// Copyright 2020 Kat Marchán (Apache-2.0)
// https://github.com/kdl-org/kdl-rs/blob/v6.7.1/src/document.rs
//
// In KDL specification, the optional version marker must appear at the very start
// of the document (after an optional BOM): `/- kdl-version <1|2>`
// https://kdl.dev/spec#compatibility
fn detect_version_marker(text: &str) -> Result<Option<KdlVersion>, FormatError> {
  let input = text.strip_prefix('\u{FEFF}').unwrap_or(text);
  let Some(first_line) = input.lines().next() else {
    return Ok(None);
  };
  let trimmed = first_line.trim_start();
  let Some(rest) = trimmed.strip_prefix("/-") else {
    return Ok(None);
  };
  let rest = rest.trim_start();
  let Some(rest) = rest.strip_prefix("kdl-version") else {
    return Ok(None);
  };
  let rest_trimmed = rest.trim_start();
  if rest.len() == rest_trimmed.len() {
    return Ok(None);
  }
  match rest_trimmed.trim() {
    "1" => Ok(Some(KdlVersion::V1)),
    "2" => Ok(Some(KdlVersion::V2)),
    version => Err(FormatError::UnsupportedVersionMarker(version.to_string())),
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn detects_version_marker() {
    assert_eq!(detect_version_marker("/- kdl-version 1").unwrap(), Some(KdlVersion::V1));
    assert_eq!(detect_version_marker("/- kdl-version 2").unwrap(), Some(KdlVersion::V2));
    assert_eq!(detect_version_marker("/-kdl-version 1").unwrap(), Some(KdlVersion::V1));
    assert_eq!(
      detect_version_marker("  /-   kdl-version   2  \nnode").unwrap(),
      Some(KdlVersion::V2)
    );
    assert_eq!(
      detect_version_marker("\u{FEFF}/- kdl-version 1\n").unwrap(),
      Some(KdlVersion::V1)
    );

    // invalid or missing version markers (ignored as regular text)
    assert_eq!(detect_version_marker("/- kdl-version1").unwrap(), None);
    assert_eq!(detect_version_marker("node 1").unwrap(), None);
    assert_eq!(detect_version_marker("// comment\n/- kdl-version 1").unwrap(), None);
    assert_eq!(detect_version_marker("\n/- kdl-version 1").unwrap(), None);
    assert_eq!(detect_version_marker("// regular comment").unwrap(), None);
    assert_eq!(detect_version_marker("/* regular block comment */").unwrap(), None);
    assert_eq!(detect_version_marker("/- other_node 1 2").unwrap(), None);

    // unsupported version markers are an error
    assert_eq!(
      detect_version_marker("/- kdl-version 3").unwrap_err().to_string(),
      "Unsupported KDL version '3' in version marker. Supported versions are '1' and '2'."
    );
    assert_eq!(
      detect_version_marker("/- kdl-version foo").unwrap_err().to_string(),
      "Unsupported KDL version 'foo' in version marker. Supported versions are '1' and '2'."
    );
  }
}
