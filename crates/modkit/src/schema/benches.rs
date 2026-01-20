use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchDef {
    pub id: String,
    pub name: String,
    pub craft_speed_mult: f32,
    pub noise_mult: f32,

    #[serde(default)]
    pub allowed_recipe_tags: Vec<String>,
}
