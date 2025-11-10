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

            // PRompt user to take the amulet to the altar
            if item.name.eq_ignore_ascii_case("Amulet") {
                println!("✨ You picked up the Amulet. You must bring it to the Sacred Altar to activate it.");
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
                        println!("✨ You place the {} on the {}. The path forward is revealed!", item.name, target);

                        if !player.flags.contains(&"tutorial_completed".to_string()) {
                            player.flags.push("tutorial_completed".to_string());
                        }

                        // If you have a level progression mechanic, mark level1 completed here
                        if !player.flags.contains(&"level1_completed".to_string()) {
                            player.flags.push("level1_completed".to_string());
                            println!("🏆 Tutorial completed! You may proceed to the next area.");
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

        if item.name.to_lowercase() == "map" {
            print_map(player, world);
        }

    } else {
        println!("You don't have a '{}' in your inventory.", item_name);
    }
}

pub fn print_map(player: &Player, world: &World) {
    println!("--- Map ---");

    let mut positions: HashMap<String, (i32, i32)> = HashMap::new();
    let mut visited: HashSet<String> = HashSet::new();
    let mut queue: VecDeque<(String, i32, i32)> = VecDeque::new();

    // Start BFS from player
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

    // Determine grid size
    let min_x = positions.values().map(|(x, _)| *x).min().unwrap_or(0);
    let max_x = positions.values().map(|(x, _)| *x).max().unwrap_or(0);
    let min_y = positions.values().map(|(_, y)| *y).min().unwrap_or(0);
    let max_y = positions.values().map(|(_, y)| *y).max().unwrap_or(0);

    let width = (max_x - min_x + 1) as usize;
    let height = (max_y - min_y + 1) as usize;
    let cell_width = world.rooms.keys().map(|id| id.len()).max().unwrap_or(5) + 4;

    let mut grid: Vec<Vec<String>> = vec![vec![" ".repeat(cell_width); width * 2 - 1]; height * 2 - 1];

    // Place rooms and connectors
    for (room_id, (x, y)) in &positions {
        let gx = ((*x - min_x) * 2) as usize;
        let gy = ((*y - min_y) * 2) as usize;

        // Room name with player marker
        let marker = if room_id == &player.current_room { "🧍 " } else { "  " };
        grid[gy][gx] = format!("{:^width$}", format!("{}{}", marker, room_id), width = cell_width);

        // Add connectors
        if let Some(room) = world.rooms.get(room_id) {
            for (dir, target) in &room.exits {
                if let Some(&(tx, ty)) = positions.get(target) {
                    let connector_x = ((x + tx - min_x * 2) as i32) / 2;
                    let connector_y = ((y + ty - min_y * 2) as i32) / 2;

                    let gx_conn = ((*x - min_x) * 2 + (tx - x)) as usize;
                    let gy_conn = ((*y - min_y) * 2 + (ty - y)) as usize;

                    if gx_conn < grid[0].len() && gy_conn < grid.len() {
                        let conn_symbol = if tx != *x { "───" } else { "│" };
                        grid[gy_conn][gx_conn] = format!("{:^width$}", conn_symbol, width = cell_width);
                    }
                }
            }
        }
    }

    // Print
    for row in grid {
        println!("{}", row.join(""));
    }
    println!("-----------");
}
