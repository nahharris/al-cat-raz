use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemStackDef {
    pub item: String,
    pub count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoiseDef {
    pub loudness: f32,
    pub kind: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecipeRequirements {
    pub needs_equipped_item_tag: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecipeDef {
    pub id: String,
    pub name: String,
    pub bench_id: String,
    pub time_s: f32,

    #[serde(default)]
    pub recipe_tags: Vec<String>,

    pub ingredients: Vec<ItemStackDef>,

    #[serde(default)]
    pub outputs: Vec<ItemStackDef>,

    pub noise: Option<NoiseDef>,
    pub requirements: Option<RecipeRequirements>,
    pub script: Option<PathBuf>,
}
