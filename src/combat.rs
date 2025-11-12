use crate::player::Player;
use crate::enemies::Enemy;
use crate::items::ItemType;
use std::process;

pub fn start_combat(player: &mut Player, enemy: &mut Enemy, previous_room: &str) -> bool {
    println!("⚔️ You encounter a {}!", enemy.name);
    println!("{}", enemy.description);

    loop {
        println!(
            "\n❤️ Your HP: {} | 💀 {}’s HP: {}",
            player.health, enemy.name, enemy.health
        );
        println!("Choose an action (attack / heal / defend / run):");

        let action = crate::utils::get_input().to_lowercase();

        let valid_action = match action.as_str() {
            "attack" => {
                let damage = player.attack_damage();
                enemy.health -= damage;
                println!("You strike the {} for {} damage!", enemy.name, damage);

                if enemy.health <= 0 {
                    println!("🎉 You defeated the {}!", enemy.name);

                    let xp_gain = enemy.attack * 5;
                    player.add_xp(xp_gain);
                    return false;
                }
                true
            }

            "heal" => {
                if let Some(pos) = player
                    .inventory
                    .iter()
                    .position(|i| matches!(i.item_type, ItemType::Healing))
                {
                    let item = player.inventory.remove(pos);
                    if let Some(amount) = item.power {
                        player.health += amount;
                        println!(
                            "💖 You use {} and restore {} HP! Current HP: {}",
                            item.name, amount, player.health
                        );
                    } else {
                        println!("💖 You use {}, but it had no effect.", item.name);
                    }
                } else {
                    println!("You have no healing items!");
                }
                true
            }

            "defend" => {
                println!("🛡️ You brace yourself!");
                true
            }

            "run" => {
                println!("🏃 You flee from the battle!");
                player.current_room = previous_room.to_string();
                return true;
            }

            _ => {
                println!("Unknown action. Type attack / heal / defend / run.");
                false // mark as invalid so enemy doesn’t attack
            }
        };

        // Only attack if player did something valid and enemy is alive
        if valid_action && enemy.health > 0 {
            let damage = enemy.attack;
            player.health -= damage;
            println!("The {} attacks you for {} damage!", enemy.name, damage);

            if player.health <= 0 {
                println!("\n💀 You have been defeated!\n");
                println!(
"⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⡀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⢀⣀⠀⠀⠀⣶⡆⠀⣰⣿⠇⣾⡿⠛⠉⠁
⠀⣠⣴⠾⠿⠿⠀⢀⣾⣿⣆⣀⣸⣿⣷⣾⣿⡿⢸⣿⠟⢓⠀⠀
⣴⡟⠁⣀⣠⣤⠀⣼⣿⠾⣿⣻⣿⠃⠙⢫⣿⠃⣿⡿⠟⠛⠁⠀
⢿⣝⣻⣿⡿⠋⠾⠟⠁⠀⠹⠟⠛⠀⠀⠈⠉⠀⠉⠀⠀⠀⠀⠀
⠀⠉⠉⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⡀⠀⠀⣀⢀⣠⣤⣴⣤⣄⠀
⠀⠀⠀⠀⣀⣤⣤⢶⣤⠀⠀⢀⣴⢃⣿⠟⠋⢹⣿⣣⣴⡿⠋⠀
⠀⠀⣰⣾⠟⠉⣿⡜⣿⡆⣴⡿⠁⣼⡿⠛⢃⣾⡿⠋⢻⣇⠀⠀
⠀⠐⣿⡁⢀⣠⣿⡇⢹⣿⡿⠁⢠⣿⠷⠟⠻⠟⠀⠀⠈⠛⠀⠀
⠀⠀⠙⠻⠿⠟⠋⠀⠀⠙⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀"
                );
                println!("\nGame Over. Thanks for playing Adventurer!\nAnother shall be sent to complete what you have failed in.");
                process::exit(0);
            }
        }
    }
}
