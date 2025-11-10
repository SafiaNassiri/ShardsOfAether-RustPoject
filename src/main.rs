mod commands;
mod player;
mod storage;
mod utils;
mod world;
mod combat;
mod enemies;

use std::error::Error;
use std::io::{stdout, Write};

use commands::{parse_command, Command};
use player::Player;
use storage::{load_game, load_world, save_game};
use utils::get_input;
use enemies::load_enemies;

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

    println!("Welcome, {} the Adventurer!", player.name);
    world::look(&player, &world);

    // --- Main game loop ---
    loop {
        print!("\n> ");
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
                    println!("\n✨ Tutorial complete! Loading Level 1 ...");
                    player.flags.push("level1_loaded".to_string());

                    // Load Level 1 explicitly
                    world = load_world("assets/level1.json").expect("Failed to load Level 1");

                    // Move player to starting room of Level 1
                    player.current_room = "forest_entrance".to_string();
                    world::look(&player, &world);
                }
            }
            Command::Use(item) => world::use_item(&item, &mut player, &world),
            Command::Inventory => println!(
                "Inventory: {:?}",
                player.inventory.iter().map(|i| &i.name).collect::<Vec<_>>()
            ),
            Command::Save => {
                save_game(&player, "save.json").unwrap();
                println!("Game saved!");
            }
            Command::Load => {
                load_game(&mut player, "save.json").unwrap();
                println!("Game loaded!");
            }
            Command::Quit => {
                println!("Farewell, brave adventurer!");
                break;
            }
            Command::Unknown(cmd) => println!("Unknown command: {}", cmd),
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
