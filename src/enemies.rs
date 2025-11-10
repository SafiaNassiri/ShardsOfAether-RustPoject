use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use once_cell::sync::Lazy;
use std::sync::RwLock;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Enemy {
    pub name: String,
    pub description: String, 
    pub health: i32,
    pub attack: i32,
    pub xp_reward: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EnemyData {
    pub enemies: HashMap<String, Enemy>,
}

// Thread-safe global storage for all enemies
pub static ENEMY_LIST: Lazy<RwLock<HashMap<String, Enemy>>> = Lazy::new(|| RwLock::new(HashMap::new()));

/// Loads enemies from JSON into global memory
pub fn load_enemies(path: &str) -> HashMap<String, Enemy> {
    let data = fs::read_to_string(path).expect("Failed to read enemies.json");
    let enemy_data: EnemyData = serde_json::from_str(&data).expect("Failed to parse enemies.json");

    let mut global_enemies = ENEMY_LIST.write().unwrap();
    *global_enemies = enemy_data.enemies.clone();

    println!("Loaded {} enemies from {}", global_enemies.len(), path);
    enemy_data.enemies
}

/// Get a cloned enemy by name
pub fn get_enemy_by_name(name: &str) -> Option<Enemy> {
    let enemies = ENEMY_LIST.read().unwrap();
    enemies.get(name).cloned()
}
