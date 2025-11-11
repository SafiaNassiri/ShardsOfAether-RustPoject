use serde::{Serialize, Deserialize};
use crate::items::Item;

#[derive(Serialize, Deserialize)]
#[derive(Clone)]
pub struct Player {
    pub name: String,
    pub current_room: String,
    pub inventory: Vec<Item>,
    pub health: i32,
    pub mana: i32,
    pub flags: Vec<String>,
    pub base_attack: i32,
    pub level: i32,
    pub xp: i32,
}

#[allow(dead_code)]
impl Player {
    pub fn new() -> Self {
        Self {
            name: "Hero".to_string(),
            current_room: "hall".to_string(),
            inventory: Vec::new(),
            health: 100,
            mana: 50,
            xp: 0,
            level: 1,
            base_attack: 10,
            flags: Vec::new(),
        }
    }

    /// XP required for next level
    pub fn xp_to_next_level(&self) -> i32 {
        50 * self.level // scales linearly; can adjust to exponential if desired
    }

    /// Called when player gains XP
    pub fn add_xp(&mut self, amount: i32) {
        self.xp += amount;
        println!("✨ You gained {} XP!", amount);

        if self.xp >= self.xp_to_next_level() {
            self.level_up();
        }
    }

    /// Handles level-up logic
    pub fn level_up(&mut self) {
        self.level += 1;
        self.xp = 0;
        self.health += 20;
        self.mana += 10;
        self.base_attack += 3;

        println!(
            "🎉 You reached Level {}!\n❤️ Max Health increased!\n🔮 Mana increased!\n⚔️ Attack power improved!",
            self.level
        );
    }

    pub fn attack_damage(&self) -> i32 {
        self.base_attack + (self.level * 2)
    }
}
