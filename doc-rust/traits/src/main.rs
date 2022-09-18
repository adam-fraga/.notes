/*
    Traits are similar to a feature often called interfaces or mixin in JS
    although with some differences.
    Trait in Rust are extremly powerfull and they can be integrated to implement
    custom behavior in EXTERNAL CRATE or even STANDART functionnality like String struct...
*/

// In a game selling stuff can be considering as shared behavior between different race
pub trait Sell {
    fn sell_weapons(&self) -> String; //Trait definition contains function signatures
    fn sell_books(&self) {
        //You can explicitly define a default behaviors for a given function in traits
        println!("Sell book to a normal price");
    }
    fn sell_armors(&self) -> String;
}

pub trait Pantheon {}
pub trait Swarm {}

// Hybrid trait can only be implement on type which implement both Pantheon and Swarm traits
pub trait Hybrid: Pantheon + Pantheon {}

pub trait Encrypt {
    fn encrypt(T: Self) -> Self;
}
#[derive(Debug)]
pub struct Human {
    pub level: u8,
    pub race: String,
    pub name: String,
    pub hp: u8,
    pub shield: u8,
}

pub struct Orc {
    pub level: u8,
    pub race: String,
    pub name: String,
    pub hp: u8,
    pub shield: u8,
}

pub struct Voidras {
    pub level: u8,
    pub race: String,
    pub name: String,
    pub hp: u8,
    pub shield: u8,
}

impl Sell for Human {
    fn sell_weapons(&self) -> String {
        format!("{} {} sell a weapon", &self.name, &self.race)
    }
    fn sell_books(&self) {
        //You can rewrite defaut behavior
        println!("Sell book prices increase by 20%");
    }

    fn sell_armors(&self) -> String {
        format!("{} {} sell an armor", &self.name, &self.race)
    }
}

impl Sell for Orc {
    fn sell_weapons(&self) -> String {
        format!("{} {} sell a weapon", &self.name, &self.race)
    }
    //You not have to implement function which implement default behavior if you want
    fn sell_armors(&self) -> String {
        format!("{} {} sell an armor", &self.name, &self.race)
    }
}

impl Pantheon for Human {}
impl Swarm for Orc {}

impl Pantheon for Voidras {}
impl Swarm for Voidras {}

impl Hybrid for Voidras {} //Voidras can implement Hybrid beacause he already impl Swar & Panth

//You can implement trait for existing crates even in std
impl Encrypt for String {
    fn encrypt(s: Self) -> Self {
        s
    }
}
impl Encrypt for &[u8] {
    fn encrypt(byte_slice: Self) -> Self {
        byte_slice
    }
}

/*
There are 2 specifics trait already implemented in rust:
- The Diplay trait:
   Used when we are formationg a string as an example the print!() maccro use "{}" to display
   data, whatever the parameter you give in parameter to substitute "{}" it have to implement
   the display trait. Strings an example implement the display trait.
- The Debug trait
   Same as the display trait it provide functionnality to debug some data as an example the println!()
   macros with "{:?}" operator call a functionnality provided by the deubg trait to debug the data
   given in 2nd parameter.
*/

fn main() {
    //Implement the Display trait for our Struct required to implement fmt method which describe how to print our
    //struct with println!()
    impl std::fmt::Display for Human {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "{} called: {}, is level {}, has {} HP and {} shield durability, ",
                self.race, self.name, self.level, self.hp, self.shield
            )
        }
    }

    let mut adam = Human {
        level: 12,
        race: String::from("Human"),
        name: String::from("Adam"),
        hp: 50,
        shield: 100,
    };

    print!("{} \n", adam); //Display trait implementation provide a way to print our struct
    print!("{:?} \n", adam); //Debug trait provide a way to display our struct for Debug
}
