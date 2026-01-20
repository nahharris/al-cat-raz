use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModDependency {
    pub mod_id: String,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModManifest {
    pub mod_id: String,
    pub name: String,
    pub version: String,
    pub mod_api_version: u32,

    #[serde(default)]
    pub dependencies: Vec<ModDependency>,

    pub description: Option<String>,
    pub authors: Option<Vec<String>>,
}
