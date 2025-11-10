use serde::{Serialize, Deserialize};
use std::collections::{HashMap, HashSet, VecDeque};
use crate::player::Player;
use crate::enemies::get_enemy_by_name; 
use crate::combat::start_combat;
use crate::colors::{colored_text, MessageType};

#[derive(Serialize, Deserialize, Clone)]
pub enum ItemType {
    Healing,      // Restores health or stamina
    Weapon,       // Increases attack
    Quest,        // Special items like Mystic Amulet
    Utility,      // Miscellaneous items like Water Flask
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Item {
    pub name: String,
    pub description: String,
    pub usable_on: Option<String>,
    pub item_type: ItemType,
    pub power: Option<i32>,
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
        // Print room name/title in a distinct color (e.g., cyan)
        println!("{}", colored_text(&room.id, MessageType::Action));

        // Print room description
        println!("\n{}", room.description);

        // Items in the room
        if !room.items.is_empty() {
            println!("You see:");
            for item in &room.items {
                println!(" - {}", colored_text(&item.name, MessageType::Item));
            }
        }

        // Exits
        if !room.exits.is_empty() {
            println!(
                "Exits: {}",
                room.exits.keys()
                    .cloned()
                    .map(|e| colored_text(&e, MessageType::Action).to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            );
        }

        // Enemy alert
        if let Some(enemy_name) = &room.enemy {
            println!("⚠️ {}", colored_text(&format!("You sense danger nearby... ({})", enemy_name), MessageType::Enemy));
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

        match item.item_type {
            ItemType::Healing => {
                if let Some(amount) = item.power {
                    player.health += amount;
                    println!("💖 You feel rejuvenated! Health +{} (current: {})", amount, player.health);
                }
            },  
            ItemType::Weapon => {
                if let Some(dmg) = item.power {
                    player.base_attack += dmg;
                    println!("⚔️ You equip {}. Attack increased by {}!", item.name, dmg);
                }
            },
            ItemType::Quest => {
                if let Some(target) = &item.usable_on {
                    if player.current_room.as_str() == target.as_str() {
                        println!("✨ You use the {} at the {}. Level complete!", item.name, target);

                        if !player.flags.contains(&"level1_completed".to_string()) {
                            player.flags.push("level1_completed".to_string());
                        }
                    } else {
                        println!("The {} glows faintly, but nothing happens here.", item.name);
                    }
                } else {
                    println!("You use the {}.", item.name);
                }
            },
            ItemType::Utility => {
                println!("You use the {}.", item.name);
                if item.name.to_lowercase() == "water flask" {
                    println!("💧 You refill the flask at the river or restore stamina.");
                }
            }
        }

        // ✅ Always print map if the item is called "map"
        if item.name.to_lowercase() == "map" {
            print_map(player, world);
        }

    } else {
        println!("You don't have a '{}' in your inventory.", item_name);
    }
}

pub fn print_map(player: &Player, world: &World) {
    use std::cmp::{min, max};

    println!("--- Map ---");

    let mut positions: HashMap<String, (i32, i32)> = HashMap::new();
    let mut visited: HashSet<String> = HashSet::new();
    let mut queue: VecDeque<(String, i32, i32)> = VecDeque::new();

    queue.push_back((player.current_room.clone(), 0, 0));
    positions.insert(player.current_room.clone(), (0, 0));

    while let Some((room_id, x, y)) = queue.pop_front() {
        if visited.contains(&room_id) { continue; }
        visited.insert(room_id.clone());

        if let Some(room) = world.rooms.get(&room_id) {
            for (dir, target) in &room.exits {
                if positions.contains_key(target) { continue; }
                let (nx, ny) = match dir.as_str() {
                    "north" => (x, y - 1),
                    "south" => (x, y + 1),
                    "east"  => (x + 1, y),
                    "west"  => (x - 1, y),
                    _ => (x, y),
                };
                positions.insert(target.clone(), (nx, ny));
                queue.push_back((target.clone(), nx, ny));
            }
        }
    }

    let min_x = positions.values().map(|(x, _)| *x).min().unwrap_or(0);
    let max_x = positions.values().map(|(x, _)| *x).max().unwrap_or(0);
    let min_y = positions.values().map(|(_, y)| *y).min().unwrap_or(0);
    let max_y = positions.values().map(|(_, y)| *y).max().unwrap_or(0);

    let width = (max_x - min_x + 1) as usize;
    let height = (max_y - min_y + 1) as usize;

    // Room grid
    let mut grid: Vec<Vec<String>> = vec![vec!["     ".to_string(); width]; height];

    // Place rooms
    let cell_width = world.rooms.keys().map(|id| id.len()).max().unwrap_or(5) + 2;
    for (room_id, (x, y)) in &positions {
        let px = (*x - min_x) as usize;
        let py = (*y - min_y) as usize;
        let marker = if room_id == &player.current_room { "🧍" } else { " " };
        grid[py][px] = format!("{:^width$}", format!("{}{}", marker, room_id), width = cell_width);
    }

    // Print row by row with arrows
    for y in 0..height {
        // Print rooms
        for x in 0..width {
            print!("{}", grid[y][x]);
            if x < width - 1 {
                // horizontal arrow
                for (room_id, (rx, ry)) in &positions {
                    if *rx == (x as i32 + min_x) && *ry == (y as i32 + min_y) {
                        if let Some(room) = world.rooms.get(room_id) {
                            if room.exits.contains_key("east") { print!("→"); } else { print!(" "); }
                        }
                    }
                }
            }
        }
        println!();

        // Print vertical arrows between rows
        if y < height - 1 {
            for x in 0..width {
                let mut arrow = " ".to_string();
                for (room_id, (rx, ry)) in &positions {
                    if *rx == (x as i32 + min_x) && *ry == (y as i32 + min_y) {
                        if let Some(room) = world.rooms.get(room_id) {
                            if room.exits.contains_key("south") { arrow = "↓".to_string(); }
                        }
                    }
                }
                print!("{:^width$}", arrow, width = cell_width);
            }
            println!();
        }
    }

    println!("-----------");
}