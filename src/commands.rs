pub enum Command {
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
    let mut parts = input.trim().splitn(2, ' '); // split into at most 2 parts
    let cmd = parts.next().unwrap_or("").to_lowercase();
    let arg = parts.next().unwrap_or("").trim().to_string();

    match cmd.as_str() {
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
