use serde::{Serialize, Deserialize};
use crate::items::Item;

#[derive(Serialize, Deserialize, Clone)]
pub struct Player {
    pub name: String,
    pub health: i32,
    pub max_health: i32,
    pub mana: i32,
    pub max_mana: i32,
    pub xp: i32,
    pub level: i32,
    pub base_attack: i32,
    pub current_room: String,
    pub inventory: Vec<Item>,
    pub flags: Vec<String>,
    pub current_level: usize,
}

impl Player {
    pub fn new() -> Self {
        Self {
            name: "Hero".to_string(),
            health: 100,
            max_health: 100,
            mana: 50,
            max_mana: 50,
            xp: 0,
            level: 1,
            base_attack: 10,
            current_room: "tutorial_hall".to_string(),
            inventory: Vec::new(),
            flags: Vec::new(),
            current_level: 0, // 0 = tutorial, 1 = level1, 2 = level2, etc.
        }
    }

    // XP needed to reach the next level
    pub fn xp_to_next_level(&self) -> i32 {
        50 * self.level
    }

    // Adds XP and automatically checks for level up
    pub fn add_xp(&mut self, amount: i32) {
        self.xp += amount;
        println!("✨ You gained {} XP!", amount);

        // Automatically level up if XP exceeds threshold
        while self.xp >= self.xp_to_next_level() {
            self.xp -= self.xp_to_next_level();
            self.level_up();
        }
    }

    // Level-up stat increases
    pub fn level_up(&mut self) {
        self.level += 1;

        // Stat increases per level
        self.max_health += 20;
        self.max_mana += 10;
        self.base_attack += 3;

        // Restore some health & mana on level-up
        self.health = self.max_health;
        self.mana = self.max_mana;

        println!(
            "🎉 You reached Level {}!\n❤️ Health restored to {}!\n🔮 Mana restored to {}!\n⚔️ Attack power increased!",
            self.level, self.max_health, self.max_mana
        );
    }

    // Damage calculation (scales with level)
    pub fn attack_damage(&self) -> i32 {
        self.base_attack + (self.level * 2)
    }

    // Ensures HP doesn’t exceed max
    pub fn heal(&mut self, amount: i32) {
        self.health = (self.health + amount).min(self.max_health);
        println!("💖 You recovered {} HP! (Current HP: {}/{})", amount, self.health, self.max_health);
    }
}
