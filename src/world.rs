    use serde::{Serialize, Deserialize};
    use crate::player::Player;
    use crate::items::{Item, ItemType};
    use crate::enemies::get_enemy_by_name; 
    use crate::combat::start_combat;
    use crate::colors::{colored_text, MessageType};
    use std::collections::{HashMap, HashSet, VecDeque};
    use itertools::Itertools;

    #[derive(Serialize, Deserialize, Clone)]
    pub struct Room {
        pub id: String,
        pub description: String,
        #[serde(default)]
        pub items: Vec<Item>,
        #[serde(default)]
        pub exits: HashMap<String, String>,
        #[serde(default)]
        pub enemy: Option<String>,
        #[serde(default)]
        pub x: i32,
        #[serde(default)]
        pub y: i32,
    }

    #[derive(Serialize, Deserialize, Clone)]
    pub struct World {
        pub rooms: HashMap<String, Room>,
        #[serde(default)]
        pub cleared_rooms: HashSet<String>,
    }

    //
    // ─── MOVEMENT ─────────────────────────────────────────────
    //
    pub fn move_player(direction: String, player: &mut Player, world: &mut World) {
        if let Some(room) = world.rooms.get(&player.current_room) {
            if let Some(next_room_id) = room.exits.get(&direction) {
                let previous_room = player.current_room.clone();
                player.current_room = next_room_id.clone();

                println!("You move {}.", direction);
                println!("🧍 You have entered: {}", player.current_room);
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
                                    world.cleared_rooms.insert(player.current_room.clone());
                                }
                            }

                            // Retreat handling
                            if ran_away {
                                player.current_room = previous_room;
                                println!("You have escaped back to {}.", player.current_room);
                            }
                        } else {
                            println!("(⚠️ Warning: Enemy '{}' not found!)", enemy_name);
                        }
                    }
                }
            } else {
                println!("You can't go that way.");
            }
        }
    }

    //
    // ─── ROOM DESCRIPTIONS ────────────────────────────────────
    //
    pub fn look(player: &Player, world: &World) {
        if let Some(room) = world.rooms.get(&player.current_room) {
            println!("{}", colored_text(&room.id, MessageType::Action));
            println!("\n{}", room.description);

            if !room.items.is_empty() {
                println!("You see:");
                for item in &room.items {
                    println!(" - {}", colored_text(&item.name, MessageType::Item));
                }
            }

            if !room.exits.is_empty() {
                let exits = room.exits.keys()
                    .map(|e| colored_text(e, MessageType::Action).to_string())
                    .collect::<Vec<_>>()
                    .join(", ");
                println!("Exits: {}", exits);
            }

            if let Some(enemy_name) = &room.enemy {
                println!("⚠️ {}", colored_text(&format!("You sense danger nearby... ({})", enemy_name), MessageType::Enemy));
            }
        }
    }

    //
    // ─── ITEM HANDLING ────────────────────────────────────────
    //
    pub fn take_item(item_name: &str, player: &mut Player, world: &mut World) {
        if let Some(room) = world.rooms.get_mut(&player.current_room) {
            if let Some(pos) = room.items.iter().position(|i| i.name.eq_ignore_ascii_case(item_name)) {
                let item = room.items.remove(pos);
                println!("You picked up: {}", colored_text(&item.name, MessageType::Item));
                player.inventory.push(item.clone());

                // Flag: picked up Amulet
                if item.name.eq_ignore_ascii_case("Amulet") {
                    println!("✨ You picked up the Amulet. Bring it to the Sacred Altar to activate it.");
                }

                // Mark this room as cleared
                world.cleared_rooms.insert(player.current_room.clone());
            } else {
                println!("There is no {} here.", item_name);
            }
        }
    }

    pub fn use_item(item_name: &str, player: &mut Player, world: &mut World) -> bool {
        let mut level_completed = false;

        if let Some(pos) = player
            .inventory
            .iter()
            .position(|i| i.name.eq_ignore_ascii_case(item_name))
        {
            let item = &player.inventory[pos];

            match item.item_type {
                ItemType::Healing => {
                    if let Some(amount) = item.power {
                        player.health += amount;
                        println!("💖 You restore {} HP! (Now at {})", amount, player.health);
                    }
                    player.inventory.remove(pos);
                }

                ItemType::Weapon => {
                    if let Some(dmg) = item.power {
                        player.base_attack += dmg;
                        println!("⚔️ Equipped {}! Attack +{}", item.name, dmg);
                    }
                    player.inventory.remove(pos);
                }

                ItemType::Quest => {
                    if let Some(target) = &item.usable_on {
                        if player.current_room == *target {
                            println!(
                                "✨ You place the {} on the {}. The path forward opens!",
                                item.name, target
                            );

                            // Remove quest item and mark room cleared
                            player.inventory.remove(pos);
                            world.cleared_rooms.insert(player.current_room.clone());

                            //
                            // LEVEL COMPLETION LOGIC
                            //

                            // Tutorial end → Level 1
                            if player.current_level == 0
                                && !player.flags.contains(&"tutorial_completed".to_string())
                            {
                                player.flags.push("tutorial_completed".to_string());
                            }

                            // Level 1 end → Level 2
                            else if player.current_level == 1
                                && !player.flags.contains(&"level1_completed".to_string())
                            {
                                player.flags.push("level1_completed".to_string());
                                println!("{}", colored_text(
                                    "🌿 The forest’s magic subsides... A desert wind begins to blow from afar.",
                                    MessageType::Info
                                ));
                            }

                            // Level 2 end → Realm of Aether
                            else if player.current_level == 2
                                && !player.flags.contains(&"level2_completed".to_string())
                            {
                                player.flags.push("level2_completed".to_string());
                                println!("{}", colored_text(
                                    "🌌 The relic hums with light. Reality bends... You are drawn into the Realm of Aether!",
                                    MessageType::Success
                                ));
                                println!("{}", colored_text(
                                    "🏆 You win! You have returned all the Shards of Aether to their rightful places, forever to be guarded by the ancients.",
                                    MessageType::Success
                                ));
                            }

                            level_completed = true;
                        } else {
                            println!("You can’t use the {} here.", item.name);
                        }
                    } else {
                        println!("That item can’t be used directly.");
                    }
                }

                ItemType::Utility => {
                    if item.name.eq_ignore_ascii_case("map") {
                        print_map(player, world);
                    } else if item.name.eq_ignore_ascii_case("water flask") {
                        println!("💧 You use the Water Flask to refresh yourself.");
                    } else {
                        println!("You use the {}.", item.name);
                    }
                }
            }
        } else {
            println!("You don't have a '{}' in your inventory.", item_name);
        }

        level_completed
    }

    //
    // ─── MAP RENDERING WITH FIXED COORDINATES ─────────────────
    //
    pub fn print_map(player: &Player, world: &World) {
        println!("--- Map ---");

        let mut positions: HashMap<String, (i32, i32)> = HashMap::new();
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();

        queue.push_back((player.current_room.clone(), 0, 0));
        positions.insert(player.current_room.clone(), (0, 0));

        while let Some((room_id, x, y)) = queue.pop_front() {
            if !visited.insert(room_id.clone()) { continue; }

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

        let (min_x, max_x) = positions.values().map(|(x, _)| *x).minmax().into_option().unwrap_or((0, 0));
        let (min_y, max_y) = positions.values().map(|(_, y)| *y).minmax().into_option().unwrap_or((0, 0));

        let width = (max_x - min_x + 1) as usize;
        let height = (max_y - min_y + 1) as usize;
        let cell_width = world.rooms.keys().map(|id| id.len()).max().unwrap_or(5) + 4;

        let mut grid = vec![vec![" ".repeat(cell_width); width * 2 - 1]; height * 2 - 1];

        for (room_id, (x, y)) in &positions {
            let gx = ((*x - min_x) * 2) as usize;
            let gy = ((*y - min_y) * 2) as usize;

            let marker = if room_id == &player.current_room { "🧍 " } else { "  " };
            grid[gy][gx] = format!("{:^width$}", format!("{}{}", marker, room_id), width = cell_width);

            if let Some(room) = world.rooms.get(room_id) {
                for (_dir, target) in &room.exits {
                    if let Some(&(tx, ty)) = positions.get(target) {
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

        for row in grid {
            println!("{}", row.join(""));
        }
        println!("-----------");
    }

    //
    // ─── LEVEL TRANSITION BANNER ─────────────────────────────
    //
    pub fn print_transition_banner(title: &str) {
        let padding = 6;
        let inner_width = title.len() + padding * 2;
        let border_top = format!("╔{}╗", "═".repeat(inner_width));
        let border_bottom = format!("╚{}╝", "═".repeat(inner_width));

        let line = format!("{:^width$}", format!("☽✧  {}  ✧☾", title), width = inner_width + 2);

        println!();
        println!("{}", colored_text(&border_top, MessageType::Info));
        println!("{}", colored_text(&line, MessageType::Info));
        println!("{}", colored_text(&border_bottom, MessageType::Info));
        println!();
    }
