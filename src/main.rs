mod commands;
mod player;
mod utils;
mod world;
mod combat;
mod enemies;
mod colors;
mod save_load;
mod items;

use std::error::Error;
use std::io::{stdout, Write};

use commands::{parse_command, Command, print_help};
use player::Player;
use utils::get_input;
use enemies::load_enemies;
use colors::{MessageType, colored_text};
use save_load::{save_game, load_game, load_world};

fn main() {
    let _enemies = load_enemies("assets/enemies.json");
    let mut player = Player::new();
    let mut world = load_level(&player).expect("Failed to load initial world");

    // Print banner
    print_current_level_banner(&player);

    println!(
        "{}",
        colored_text(
            &format!("Welcome, {} the Adventurer!", player.name),
            MessageType::Info
        )
    );
    world::look(&player, &world);

    // === MAIN GAME LOOP ===
    loop {
        print!("{}", colored_text("\n> ", MessageType::Action));
        stdout().flush().unwrap();

        let input = get_input().to_lowercase();
        let command = parse_command(&input);

        match command {
            Command::Help => print_help(),
            Command::Go(dir) => world::move_player(dir, &mut player, &mut world),
            Command::Look => world::look(&player, &world),

            Command::Take(item) => {
                world::take_item(&item, &mut player, &mut world);
                handle_level_progression(&mut player, &mut world);
                // println!("DEBUG: flags = {:?}", player.flags);
            }

            Command::Use(item) => {
                let completed = world::use_item(&item, &mut player, &mut world);

                // println!("DEBUG: current_level = {}", player.current_level);
                // println!("DEBUG: flags = {:?}", player.flags);

                if completed {
                    handle_level_progression(&mut player, &mut world);
                    // println!("DEBUG: flags = {:?}", player.flags);
                }
            }
            Command::Inventory => {
                let inventory_display = player
                    .inventory
                    .iter()
                    .map(|i| i.name.as_str())
                    .collect::<Vec<_>>()
                    .join(", ");
                println!(
                    "{}",
                    colored_text(
                        &format!("Inventory: [{}]", inventory_display),
                        MessageType::Item
                    )
                );
            }

            Command::Save => {
                save_game(&player, &world, "save.json").unwrap();
                println!("{}", colored_text("Game saved!", MessageType::Info));
            }

            Command::Load => {
                load_game(&mut player, &mut world, "save.json").unwrap();
                println!("{}", colored_text("Game loaded!", MessageType::Info));
            }

            Command::Quit => {
                println!("{}", colored_text("Farewell, brave adventurer!", MessageType::Info));
                break;
            }

            Command::Unknown(cmd) => {
                println!("{}", colored_text(&format!("Unknown command: {}", cmd), MessageType::Warning));
            }
        }
    }
}

// === LEVEL LOADING ===

// Load the appropriate world file for the player's level
fn load_level(player: &Player) -> Result<world::World, Box<dyn Error>> {
    match player.current_level {
        0 => Ok(load_world("assets/tutorial.json")?),
        n => Ok(load_world(&format!("assets/level{}.json", n))?),
    }
}

// Display a banner based on current level number
fn print_current_level_banner(player: &Player) {
    match player.current_level {
        0 => world::print_transition_banner("Tutorial: The Guild Hall"),
        1 => world::print_transition_banner("Level 1: The Emerald Forest"),
        2 => world::print_transition_banner("Level 2: The Desert Sands"),
        3 => world::print_transition_banner("Sanctum of Aether: You Win!"),
        _ => world::print_transition_banner("The Realm of Aether"),
    }
}   

// === LEVEL PROGRESSION ===

fn handle_level_progression(player: &mut Player, world: &mut world::World) {
    // Tutorial → Level 1
    if player.flags.contains(&"tutorial_completed".to_string()) 
        && !player.flags.contains(&"level1_loaded".to_string()) 
    {
        world::print_transition_banner("Level 1: The Emerald Forest");
        player.flags.push("level1_loaded".to_string());

        *world = load_world("assets/level1.json").expect("Failed to load Level 1");
        player.current_room = "forest_entrance".to_string();

        player.current_level = 1;

        world::look(player, world);
    }

    // Level 1 → Level 2
    if player.flags.contains(&"level1_completed".to_string())
        && !player.flags.contains(&"level2_loaded".to_string())
    {
        world::print_transition_banner("Level 2: The Desert Sands");
        player.flags.push("level2_loaded".to_string());

        *world = load_world("assets/level2.json").expect("Failed to load Level 2");
        player.current_room = "desert_edge".to_string();
        player.current_level = 2;
        world::look(player, world);
    }

    // Level 2 → Sanctum
    if player.flags.contains(&"level2_completed".to_string())
        && !player.flags.contains(&"sanctum_loaded".to_string())
    {
        world::print_transition_banner("Sanctum of Aether: You Win!");
        player.flags.push("sanctum_loaded".to_string());

        let saved_inventory = player.inventory.clone();
        *world = load_world("assets/sanctum.json").expect("Failed to load Sanctum");
        player.inventory = saved_inventory;
        player.current_room = "sanctum".to_string();
        world::look(player, world);
    }
}