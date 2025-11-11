use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub enum ItemType {
    Healing,
    Weapon,
    Quest,
    Utility,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Item {
    pub name: String,
    pub item_type: ItemType,
    pub power: Option<i32>,
    pub usable_on: Option<String>,
}
