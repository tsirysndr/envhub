use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

pub enum ConfigFormat {
    TOML,
    HCL,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct File {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Packages {
    pub packages: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct RtxParams {
    pub packages: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct RustupParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub toolchains: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Configuration {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub packages: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets: Option<Vec<String>>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "hcl::ser::block"
    )]
    pub homebrew: Option<Packages>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "hcl::ser::block"
    )]
    pub pkgx: Option<Packages>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "hcl::ser::block"
    )]
    pub devbox: Option<Packages>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "hcl::ser::block"
    )]
    pub nix: Option<Packages>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "hcl::ser::block"
    )]
    pub envs: Option<IndexMap<String, String>>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        rename = "file",
        serialize_with = "hcl::ser::labeled_block"
    )]
    pub files: Option<IndexMap<String, File>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symlink_manager: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_manager: Option<String>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "hcl::ser::block"
    )]
    pub rtx: Option<RtxParams>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "hcl::ser::block"
    )]
    pub rustup: Option<RustupParams>,
}
