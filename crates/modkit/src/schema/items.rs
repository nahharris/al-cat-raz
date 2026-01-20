use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EquipSlot {
    Head,
    Body,
    Feet,
    Tail,
    Teeth,
    WeaponMain,
    WeaponOff,
    Trinket1,
    Trinket2,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GadgetSlot {
    Edge,
    Handle,
    Plating,
    Lining,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeaponDef {
    pub damage: f32,
    pub cooldown: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArmorDef {
    pub defense: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StackableDef {
    pub max: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StatId {
    Damage,
    Cooldown,
    Defense,
    Durability,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatModifier {
    pub stat: StatId,
    pub factor: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GadgetDef {
    #[serde(default, alias = "slot", deserialize_with = "deserialize_gadget_slots")]
    pub slots: Vec<GadgetSlot>,

    #[serde(default)]
    pub modifiers: Vec<StatModifier>,

    pub script: Option<PathBuf>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GadgetHostDef {
    pub slot: GadgetSlot,
    pub count: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EquipableDef {
    pub slot: EquipSlot,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DamageableDef {
    pub durability: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ItemComponent {
    Stackable(StackableDef),
    Equipable(EquipableDef),
    Weapon(WeaponDef),
    Armor(ArmorDef),
    Damageable(DamageableDef),
    Gadget(GadgetDef),
    GadgetHost(GadgetHostDef),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "Item")]
pub struct ItemDef {
    pub id: String,
    pub name: String,
    pub description: String,

    #[serde(default)]
    pub tags: Vec<String>,

    #[serde(default)]
    pub components: Vec<ItemComponent>,
}

fn deserialize_gadget_slots<'de, D>(deserializer: D) -> Result<Vec<GadgetSlot>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum SlotList {
        One(GadgetSlot),
        Many(Vec<GadgetSlot>),
    }

    match SlotList::deserialize(deserializer)? {
        SlotList::One(slot) => Ok(vec![slot]),
        SlotList::Many(slots) => Ok(slots),
    }
}
