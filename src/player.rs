use serde::{Serialize, Deserialize};
use crate::items::Item;

#[derive(Serialize, Deserialize, Clone)]
pub struct Player {
    pub name: String,
    pub health: i32,
    pub mana: i32,
    pub xp: i32,
    pub level: i32,
    pub base_attack: i32,
    pub current_room: String,
    pub inventory: Vec<Item>,
    pub flags: Vec<String>,
    pub current_level: usize, // level of the world (not XP level)
}

impl Player {
    pub fn new() -> Self {
        Self {
            name: "Hero".to_string(),
            health: 100,
            mana: 50,
            xp: 0,
            level: 1,
            base_attack: 10,
            current_room: "tutorial_hall".to_string(),
            inventory: Vec::new(),
            flags: Vec::new(),
            current_level: 0, // 0 = tutorial, 1 = level1, 2 = level2, etc.
        }
    }

    /// XP required for next level
    pub fn xp_to_next_level(&self) -> i32 {
        50 * self.level // scales linearly; tweak to exponential if desired
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

    /// Returns player’s damage output (based on attack and XP level)
    pub fn attack_damage(&self) -> i32 {
        self.base_attack + (self.level * 2)
    }
}
