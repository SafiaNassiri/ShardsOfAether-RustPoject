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
    // --- Load dynamic data ---
    let _enemies = load_enemies("assets/enemies.json");

    let mut player = Player {
        name: "Hero".to_string(),
        current_room: "tutorial_hall".to_string(),
        inventory: Vec::new(),
        health: 100,
        mana: 50,
        flags: Vec::new(),
        base_attack: 10,
        level: 1,
        xp: 0,
    };

    // --- Load the initial level ---
    let mut world = load_level(&player).expect("Failed to load initial world");

    println!("{}", colored_text(&format!("Welcome, {} the Adventurer!", player.name), MessageType::Info));
    world::look(&player, &world);

    // --- Main game loop ---
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

                // Check for tutorial completion after picking up quest items
                handle_level_progression(&mut player, &mut world);
            }

            Command::Use(item) => {
                let completed = world::use_item(&item, &mut player, &mut world);

                // If using the item completes the level
                if completed {
                    handle_level_progression(&mut player, &mut world);
                }
            }

            Command::Inventory => {
                let inventory_display = player
                    .inventory
                    .iter()
                    .map(|i| i.name.as_str())
                    .collect::<Vec<_>>()
                    .join(", ");
                println!("{}", colored_text(&format!("Inventory: [{}]", inventory_display), MessageType::Item));
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

/// Load the initial world based on player flags
fn load_level(player: &Player) -> Result<world::World, Box<dyn Error>> {
    if !player.flags.contains(&"tutorial_completed".to_string()) {
        Ok(load_world("assets/tutorial.json")?)
    } else if !player.flags.contains(&"level1_completed".to_string()) {
        Ok(load_world("assets/level1.json")?)
    } else {
        Ok(load_world("assets/world.json")?)
    }
}

/// Handles level progression and automatically loads next level when appropriate
fn handle_level_progression(player: &mut Player, world: &mut world::World) {
    // Tutorial → Level 1
    if player.flags.contains(&"tutorial_completed".to_string())
        && !player.flags.contains(&"level1_loaded".to_string())
    {
        println!("{}", colored_text("\n✨ Tutorial complete! Loading Level 1 ...", MessageType::Info));
        player.flags.push("level1_loaded".to_string());

        *world = load_world("assets/level1.json").expect("Failed to load Level 1");
        player.current_room = "forest_entrance".to_string();
        world::look(player, world);
    }

    // Level 1 → Full game world (example, extend as needed)
    if player.flags.contains(&"level1_completed".to_string())
        && !player.flags.contains(&"world_loaded".to_string())
    {
        println!("{}", colored_text("\n✨ Level 1 complete! Loading full world ...", MessageType::Info));
        player.flags.push("world_loaded".to_string());

        *world = load_world("assets/world.json").expect("Failed to load main world");
        // Set starting room for main world
        player.current_room = "starting_village".to_string();
        world::look(player, world);
    }
}
