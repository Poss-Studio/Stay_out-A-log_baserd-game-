use crate::Player;
use std::io;
pub fn hospital_union(player: &mut Player) {
    println!("Welcome to the Union hospital!");
    let check = player.money;
    loop {
        if check >= 100 && player.hp <= 50 {
            println!("Are you want to get a aid? (Y/N)");
            let mut ic = String::new();
            io::stdin().read_line(&mut ic).unwrap();
            match ic.trim() {
                "Y" => {
                    player.hp = 100;
                    println!("Your hp is {}", player.hp);
                    player.money -= 100;
                    println!("Now you have {} in your pocket", player.money);
                    break;
                }
                "N" => break,
                &_ => continue,
            }
        }
        if check < 100 && player.hp <= 50 {
            player.hp = 100;
            println!("Your hp is {}", player.hp);
            break;
        }
        if player.hp > 70 && check > 100 {
            println!("You are fine, Just go away");
            break;
        }
    }
}
