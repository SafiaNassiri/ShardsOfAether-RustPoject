use crate::player::Player;
use crate::enemies::Enemy;
use crate::items::ItemType;

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

        match action.as_str() {
            "attack" => {
                let damage = player.attack_damage();
                enemy.health -= damage;
                println!("You strike the {} for {} damage!", enemy.name, damage);

                if enemy.health <= 0 {
                    println!("🎉 You defeated the {}!", enemy.name);

                    // Award XP on victory
                    let xp_gain = enemy.attack * 5; // XP reward scales by enemy strength
                    player.add_xp(xp_gain);

                    return false; // combat over, not running
                }
            }

            "heal" => {
                // Find first Healing item in inventory
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
            }

            "defend" => {
                println!("🛡️ You brace yourself!");
                // Optional: reduce next enemy attack by half
            }

            "run" => {
                println!("🏃 You flee from the battle!");
                player.current_room = previous_room.to_string(); // move back
                return true; // signal that player ran away
            }

            _ => println!("Unknown action. Type attack / heal / defend / run."),
        }

        // Enemy attacks if still alive
        if enemy.health > 0 {
            let damage = enemy.attack; // can be modified by player defense logic
            player.health -= damage;
            println!("The {} attacks you for {} damage!", enemy.name, damage);

            if player.health <= 0 {
                println!("💀 You have been defeated!");
                return false; // combat ends
            }
        }
    }
}
