//Options are really usefull to work with optionnal values
// Bydefault Rust import Option::Some and Option::None so you don't have to import them with use statement.

//Lookup a player by is id
fn lookup_player(id: u32) -> Option<String> {
    if id == 1 {
        return Some("Aeris".to_string());
    }
    None
}


fn run_game() {
    let player = match lookup_player(1) {
        Some(p) => p,
        None => return,
    };
}

// The pattern above is so common that rust give you syntax sugar to do the same with much less lines of code
fn run_game_common_way() -> Option<()> {
    let player = lookup_player(1)?; //Question mark return None if lookup player return None
    println!("{player}");
    Some(())
}

fn main() {}
