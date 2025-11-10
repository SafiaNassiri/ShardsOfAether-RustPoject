mod commands;
mod player;
mod storage;
mod utils;
mod world;
mod combat;
mod enemies;
mod colors;

use std::error::Error;
use std::io::{stdout, Write};

use commands::{parse_command, Command};
use player::Player;
use storage::{load_game, load_world, save_game};
use utils::get_input;
use enemies::load_enemies;
use colors::{MessageType, colored_text};

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
            Command::Go(dir) => world::move_player(dir, &mut player, &mut world),
            Command::Look => world::look(&player, &world),
            Command::Take(item) => {
                world::take_item(&item, &mut player, &mut world);

                // Tutorial completion triggers Level 1 load
                if player.flags.contains(&"tutorial_completed".to_string())
                    && !player.flags.contains(&"level1_loaded".to_string())
                {
                    println!("{}", colored_text("\n✨ Tutorial complete! Loading Level 1 ...", MessageType::Info));
                    player.flags.push("level1_loaded".to_string());

                    // Load Level 1 explicitly
                    world = load_world("assets/level1.json").expect("Failed to load Level 1");

                    // Move player to starting room of Level 1
                    player.current_room = "forest_entrance".to_string();
                    world::look(&player, &world);
                }
            }
            Command::Use(item) => world::use_item(&item, &mut player, &world),
            Command::Inventory => {
                let inventory_display = player
                    .inventory
                    .iter()
                    .map(|i| i.name.as_str()) // convert &String -> &str
                    .collect::<Vec<_>>()
                    .join(", ");
                println!("{}", colored_text(&format!("Inventory: [{}]", inventory_display), MessageType::Item));
            }
            Command::Save => {
                save_game(&player, "save.json").unwrap();
                println!("{}", colored_text("Game saved!", MessageType::Info));
            }
            Command::Load => {
                load_game(&mut player, "save.json").unwrap();
                println!("{}", colored_text("Game loaded!", MessageType::Info));
            }
            Command::Quit => {
                println!("{}", colored_text("Farewell, brave adventurer!", MessageType::Info));
                break;
            }
            Command::Unknown(cmd) => println!("{}", colored_text(&format!("Unknown command: {}", cmd), MessageType::Warning)),
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
