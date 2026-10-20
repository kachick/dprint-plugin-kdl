use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Configuration {
  pub kdl_version: KdlVersion,
}

// Why no `auto` version or fallback parsing?
// 1. Silent syntax changes: Many KDL v1 documents are valid KDL v2 syntax.
//    If we try v2 first with fallback to v1, valid v1 documents (e.g. quoted strings)
//    will succeed in v2 parsing and be reformatted with v2 rules (quotes removed),
//    breaking tools that expect v1.
// 2. Performance: Trying v2 and then v1 doubles the parsing time on errors.
// 3. Clear alternatives: Users can set `kdlVersion` in config / overrides,
//    or add `/- kdl-version 1` or `2` at the top of the file.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
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
