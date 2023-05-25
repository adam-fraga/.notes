/*
    Match is usefull when it comes to manage multiple case of errors or comparison.
    Most part of the time it's use with Enumeration.
    the code will not compile if the totality of the variance for a given enum are not
    covered in match branches.
*/

// Enum can contain any type

#[derive(Debug)]
enum Types {
    Bool(boolean),
    Slice(&str),
    StructLiteral { name: &str, hp: u32 },
    Array([u32; 5]),
    OtherEnum(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    _Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    /*
        Match is a controll flow which compare a value against different pattern.
        Contrarly to if statement match can return any type not only bool (here the type is Coin)
    */
    fn value_in_cents(coin: Coin) -> u8 {
        /*
            Arm of branch is composed by a pattern "Coin", separator "=>" and code to exec (1)
            the code after the separator is an expression it will be return by match if the pattern
            is matching.
        */

        match coin {
            Coin::Penny => {
                println!("Lucky penny!"); // use {} for multiple lines branches
                1 // coin match with Coin::Penny
            }
            Coin::Nickel => 5,
            Coin::Dime => 10,
            //State is here an other type of variable (enum UsState)
            //We can then specify a coin with more specificities and extract or display state value
            Coin::Quarter(state) => {
                println!("State quarter from {:?}!", state);
                25
            }
        }
    }

    //Match with Penny
    let coin: u8 = value_in_cents(Coin::Penny);
    //Match with Quarter and retrieve Alabama Type
    let coin2: u8 = value_in_cents(Coin::Quarter(UsState::Alabama));
    println!("{}", coin);
    println!("{}", coin2);

    //-------------------------------------------------------------------------------------------

    //Match with option
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            Some(i) => Some(i + 1), // add one if x exist
            None => None,           //nothing if x does not exist)
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    //-------------------------------------------------------------------------------------------

    //Match with default case
    let dice_roll = 9; //here user get dice roll 9
    match dice_roll {
        //It's possible to have litteral in pattern like here 3 and 7
        3 => add_fancy_hat(),    //user don't get fancy hat
        7 => remove_fancy_hat(), //user don't remove his fancy hat
        //other here match with all patterns is not 3 or 7 (must be call at last)
        other => move_player(other), //move player is called user move
    }

    fn add_fancy_hat() {
        println!("You win a Fancy hat");
    }
    fn remove_fancy_hat() {
        println!("You lose your Fancy hat");
    }
    fn move_player(num_spaces: u8) {
        println!("User move from {} position", num_spaces);
    }

    //-------------------------------------------------------------------------------------------

    /*
    Match pattern can also be use with other value than enum like number, string and so on
    If the value is not necessary you can use  the "_" which does not bind value
    This time we don't need the dice roll result we just said the user must reroll
    */
    let dice_roll2 = 9;

    match dice_roll2 {
        3 => add_fancy_hat2(),
        13..=19 => println!("Dice cannot contain that number"), //Range matching including 19
        7 => remove_fancy_hat2(),
        _ => reroll(), // Similar to default in switch statement (All the other case will match this pattern)
    }

    fn add_fancy_hat2() {
        println!("You win a Fancy hat");
    }
    fn remove_fancy_hat2() {
        println!("You Loose a Fancy hat");
    }
    fn reroll() {
        println!("Reroll untill you get 3 or 7");
    }

    //-------------------------------------------------------------------------------------------

    //If we does not need specify nothin else append we use ()
    //(empty tuple type)
    let dice_roll3 = 9;
    match dice_roll3 {
        3 => add_fancy_hat3(),
        7 => remove_fancy_hat3(),
        2 | 8 => println!("You move 2 or 8 position "),
        _ => (), // if no branch match do nothing
    }

    fn add_fancy_hat3() {
        println!("You Win a Fancy hat");
    }
    fn remove_fancy_hat3() {
        println!("You Loose a Fancy hat");
    }
}
