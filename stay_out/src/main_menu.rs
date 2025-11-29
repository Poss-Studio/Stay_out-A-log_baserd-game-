use crate::Player;
use std::io::stdin;
fn chose() {
    loop {
        println!("\n 1. Into the BackGround \n 2.Into the main Game \n 3.Load the game \n 4.exit");
        let mut ch = String::new();
        stdin().read_line(&mut ch).unwrap();
        match ch.trim() {
            "1" => {}
            "2" => {}
            "3" => {}
            "4" => break,
            &_ => continue,
        }
    }
}
pub fn main_menu(player: &mut Player) {
    println!("Welcome to the Stay_out Story Mode");
    loop {
        println!("Are you a newer? (Y/N)");
        let mut ss = String::new();
        stdin().read_line(&mut ss).unwrap();
        match ss.trim() {
            "Y" => {
                println!("press your player name to shuttle");
                stdin().read_line(&mut player.name).unwrap();
                println!("Welcome ,{}", player.name);
                println!("{:#?}", player);
                chose();
                break;
            }
            "N" => {
                player.name = String::from("Rexim");
                println!("Welcome,{}", player.name);
                println!("{:#?}", player);
                break;
            }
            &_ => continue,
        }
    }
}
