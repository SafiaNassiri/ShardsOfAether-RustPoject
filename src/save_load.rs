use serde::{Serialize, Deserialize};
use std::fs::{self, File};
use std::io::{Read, Write};
use std::error::Error;
use crate::{player::Player, world::World};

#[derive(Serialize, Deserialize, Clone)]
pub struct SaveData {
    pub player: Player,
    pub world: World,
}

pub fn save_game(player: &Player, world: &World, path: &str) -> Result<(), Box<dyn Error>> {
    let data = SaveData {
        player: player.clone(),
        world: world.clone(),
    };
    let json = serde_json::to_string_pretty(&data)?;
    let mut file = File::create(path)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}

pub fn load_game(player: &mut Player, world: &mut World, path: &str) -> Result<(), Box<dyn Error>> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let data: SaveData = serde_json::from_str(&contents)?;
    *player = data.player;
    *world = data.world;
    Ok(())
}

pub fn load_world(path: &str) -> Result<World, Box<dyn Error>> {
    let data = fs::read_to_string(path)?;
    let world: World = serde_json::from_str(&data)?;
    Ok(world)
}
