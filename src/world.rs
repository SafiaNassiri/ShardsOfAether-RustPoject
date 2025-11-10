use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use crate::player::Player;
use crate::enemies::get_enemy_by_name; 
use crate::combat::start_combat;

#[derive(Serialize, Deserialize, Clone)]
pub struct Item {
    pub name: String,
    pub description: String,
    pub usable_on: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Room {
    pub id: String,
    pub description: String,
    pub exits: HashMap<String, String>,
    pub items: Vec<Item>,
    pub enemy: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct World {
    pub rooms: HashMap<String, Room>,
}

// ---------------------------
// GAME LOGIC FUNCTIONS
// ---------------------------

// Move the player between rooms
pub fn move_player(direction: String, player: &mut Player, world: &mut World) {
    if let Some(room) = world.rooms.get(&player.current_room) {
        if let Some(next_room_id) = room.exits.get(&direction) {
            // Save previous room
            let previous_room = player.current_room.clone();

            // Move player
            player.current_room = next_room_id.clone();
            println!("You move {}.", direction);

            // Print concise room name
            println!("🧍 You have entered: {}", player.current_room);

            // Show full description
            look(player, world);

            // Trigger combat if enemy exists
            if let Some(next_room) = world.rooms.get(&player.current_room) {
                if let Some(enemy_name) = &next_room.enemy {
                    if let Some(mut enemy) = get_enemy_by_name(enemy_name) {
                        println!("\n⚔️ A wild {} appears!", enemy.name);
                        let ran_away = start_combat(player, &mut enemy, &previous_room);

                        // Remove enemy if defeated
                        if enemy.health <= 0 {
                            println!("{} has been defeated!", enemy.name);
                            if let Some(room_mut) = world.rooms.get_mut(&player.current_room) {
                                room_mut.enemy = None;
                            }
                        }

                        // Move back if ran away
                        if ran_away {
                            player.current_room = previous_room;
                            println!("You have escaped back to {}.", player.current_room);
                            return;
                        }
                    } else {
                        println!("(Warning: Enemy '{}' not found!)", enemy_name);
                    }
                }
            }
        } else {
            println!("You can't go that way.");
        }
    }
}

// Describe the current room
pub fn look(player: &Player, world: &World) {
    if let Some(room) = world.rooms.get(&player.current_room) {
        println!("\n{}", room.description);

        if !room.items.is_empty() {
            println!("You see:");
            for item in &room.items {
                println!(" - {}", item.name);
            }
        }

        if !room.exits.is_empty() {
            println!(
                "Exits: {}",
                room.exits.keys().cloned().collect::<Vec<_>>().join(", ")
            );
        }

        if let Some(enemy_name) = &room.enemy {
            println!("⚠️ You sense danger nearby... ({})", enemy_name);
        }
    }
}

// Pick up an item from the current room
pub fn take_item(item_name: &str, player: &mut Player, world: &mut World) {
    if let Some(room) = world.rooms.get_mut(&player.current_room) {
        if let Some(pos) = room.items.iter().position(|i| i.name.eq_ignore_ascii_case(item_name)) {
            let item = room.items.remove(pos);
            println!("You picked up: {}", item.name);
            player.inventory.push(item.clone());

            // Tutorial completion condition
            if item.name.eq_ignore_ascii_case("Amulet") && !player.flags.contains(&"tutorial_completed".to_string()) {
                println!("✨ The amulet glows faintly... You feel a new path has opened.");
                player.flags.push("tutorial_completed".to_string());
            }
        } else {
            println!("There is no {} here.", item_name);
        }
    }
}

// Use an item in the inventory
pub fn use_item(item_name: &str, player: &mut Player, world: &World) {
    if let Some(pos) = player.inventory.iter().position(|i| i.name.eq_ignore_ascii_case(item_name)) {
        let item = &player.inventory[pos];

        match item.name.to_lowercase().as_str() {
            "map" => {
                println!("🗺️ You unfold the map. Here's where you   are:");
                print_map(player, world);
            },
            "health potion" => {
                player.health += 20;
                println!("You feel rejuvenated! ❤️ Health restored to {}", player.health);
            },
            _ => println!("You use the {}.", item.name),
        }
    } else {
        println!("You don't have a '{}' in your inventory.", item_name);
    }
}

pub fn print_map(player: &Player, world: &World) {
    println!("--- Map ---");
    for (room_id, room) in &world.rooms {
        // Marker for player's current location
        let marker = if room_id == &player.current_room { "🧍 You are here: " } else { " " };

        // Collect items relevant to completing the level
        let mut hints = Vec::new();
        for item in &room.items {
            if ["Amulet", "Healing Herb", "map"].contains(&item.name.as_str()) {
                hints.push(item.name.clone());
            }
        }

        // Print compact info line
        if hints.is_empty() {
            println!("{} {}", marker, room_id);
        } else {
            println!("{} {} [{}]", marker, room_id, hints.join(", "));
        }
    }
    println!("-----------");
}
