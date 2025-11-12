pub enum Command {
    Help,
    Go(String),
    Look,
    Take(String),
    Use(String),
    Inventory,
    Save,
    Load,
    Quit,
    Unknown(String),
}

pub fn parse_command(input: &str) -> Command {
    let mut parts = input.trim().splitn(2, ' ');
    let cmd = parts.next().unwrap_or("").to_lowercase();
    let arg = parts.next().unwrap_or("").trim().to_string();

    match cmd.as_str() {
        "help" => Command::Help,
        "go" | "move" => Command::Go(arg),
        "look" => Command::Look,
        "take" => Command::Take(arg),
        "use" => Command::Use(arg),
        "inventory" | "inv" => Command::Inventory,
        "save" => Command::Save,
        "load" => Command::Load,
        "quit" | "exit" => Command::Quit,
        _ => Command::Unknown(input.to_string()),
    }
}

pub fn print_help() {
    println!("📝 Available Commands:");
    println!("  help             - Show this help message");
    println!("  go <direction>   - Move in a direction (north, south, east, west)");
    println!("  look             - Look around the current room");
    println!("  take <item>      - Pick up an item");
    println!("  use <item>       - Use an item from your inventory");
    println!("  inventory / inv  - Show your inventory");
    println!("  save             - Save your game progress");
    println!("  load             - Load a saved game");
    println!("  quit / exit      - Quit the game");
}
