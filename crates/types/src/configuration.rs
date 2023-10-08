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
pub struct Configuration {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub packages: Option<Vec<String>>,
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
}
