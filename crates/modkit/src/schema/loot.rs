use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LootEntryDef {
    pub weight: u32,
    pub item: String,
    pub min: u32,
    pub max: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LootTableDef {
    pub id: String,
    pub rolls: u32,
    pub entries: Vec<LootEntryDef>,
}
