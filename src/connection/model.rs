use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct Version {
    pub server: String,
    pub version: String,
    pub license: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DatabaseInfo {
    pub name: String,
    pub id: String,
    pub path: String,
    pub is_system: bool,
}
