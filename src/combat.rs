use crate::player::Player;
use crate::enemies::Enemy;

pub fn start_combat(player: &mut Player, enemy: &mut Enemy, previous_room: &str) -> bool {
    println!("You encounter a {}!", enemy.name);
    println!("{}", enemy.description);

    loop {
        println!("\n❤️ Your HP: {} | 💀 {}’s HP: {}", player.health, enemy.name, enemy.health);
        println!("Choose an action (attack / defend / run):");

        let action = crate::utils::get_input().to_lowercase();

        match action.as_str() {
            "attack" => {
                enemy.health -= player.base_attack;
                println!("You strike the {} for {} damage!", enemy.name, player.base_attack);
                if enemy.health <= 0 {
                    println!("You defeated the {}!", enemy.name);
                    return false; // combat over, not running
                }
            }
            "defend" => println!("🛡️ You brace yourself!"),
            "run" => {
                println!("🏃 You flee from the battle!");
                player.current_room = previous_room.to_string(); // move back
                return true; // signal that player ran away
            }
            _ => println!("Unknown action. Type attack / defend / run."),
        }

        // Enemy attacks if still alive
        if enemy.health > 0 {
            let damage = enemy.attack; // customize damage calculation
            player.health -= damage;
            println!("The {} attacks you for {} damage!", enemy.name, damage);
        }
    }
}
