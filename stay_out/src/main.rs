mod hospital;
mod main_menu;
#[derive(Debug)]
struct Player {
    hp: i32,
    name: String,
    money: i32,
}
impl Player {
    fn new() -> Player {
        Player {
            hp: 100,
            name: String::new(),
            money: 1000,
        }
    }
}
fn main() {
    let mut player = Player::new();
    main_menu::main_menu(&mut player);
}
