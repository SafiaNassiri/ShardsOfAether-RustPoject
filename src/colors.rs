use colored::*;

// Enum for different message types
#[allow(dead_code)]
pub enum MessageType {
    Info,
    Warning,
    Error,
    Item,
    Enemy,
    Action,
    Exit,
}

// Central function to color text based on type
pub fn colored_text(message: &str, msg_type: MessageType) -> ColoredString {
    match msg_type {
        MessageType::Info => message.white(),
        MessageType::Warning => message.yellow().bold(),
        MessageType::Error => message.red().bold(),
        MessageType::Item => message.cyan().bold(),
        MessageType::Enemy => message.red(),
        MessageType::Action => message.green().bold(),
        MessageType::Exit => message.blue(),
    }
}
