use crate::world::World;
use crate::player::Player;
use std::fs;
use serde_json;
use std::error::Error;

pub fn save_game(player: &Player, path: &str) -> Result<(), Box<dyn Error>> {
    let data = serde_json::to_string_pretty(player)?;
    fs::write(path, data)?;
    println!("Game saved!");
    Ok(())
}

pub fn load_game(player: &mut Player, path: &str) -> Result<(), Box<dyn Error>> {
    let data = fs::read_to_string(path)?;
    *player = serde_json::from_str(&data)?;
    println!("Game loaded!");
    Ok(())
}

pub fn load_world(path: &str) -> Result<World, Box<dyn Error>> {
    let data = fs::read_to_string(path)?;
    let world: World = serde_json::from_str(&data)?;
    Ok(world)
}