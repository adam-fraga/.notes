#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    let config_max = Some(3u8); //concise (3 unsigned int 8bit)
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }

    //Same code with if let if let is syntaxic sugar for match
    //When we want to ignore all patterns except one
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    //Exemple with if let and else
    let mut count = 0;
    let coin = Coin::Penny;
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }

    let mut count2 = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
}
