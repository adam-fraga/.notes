//When you want an object that has a known set of properties, you want
//a more structured data type.

#[derive(Debug)] //Fastest way to implement Debug trait for each field
struct Human {
    name: String,
    eyes: String,
    age: u8,
    hungry: bool,
}

//Implement a method and associate function to a struct
impl Human {
    // By convention we use new with an associate function as a constructor
    //Self in return value with S "Uppercase" is an alias for Human (Type)
    fn _new(name: String, eyes: String, age: u8, hungry: bool) -> Self {
        //Rust allow this syntaxt if the struct prop name match with arg name
        return Human {
            name, //Instead of name = name...
            eyes,
            age,
            hungry,
        };
    }
    // Associate function does not contain self and are called with "::"
    // Associate func are same as static method in OOP)
    fn answer() -> u8 {
        42
    }
    // Method contain self keyword and are related to instances of struct
    // self keyword = this keyword in OOP
    fn say_hello(&self) {
        println!("Hi everyone my name is {}", &self.name);
    }
}

//Tuple struct are usefull when you had to create a struct with same datatype

//Definition
struct Color(i32, i32, i32);
struct Position(i32, i32, i32);

// Empty tupple struct contain unit value usefull for trait
struct AlwaysEqual;

fn main() {
    //Instance
    let mut adam = Human {
        //Struct field cannot be mutable the entire struct must be if you want to alter data
        name: String::from("Adam"),
        eyes: String::from("Brown"),
        age: 31,
        hungry: false,
    };

    //Display struct
    println!("{:?}", adam); //Possible because of debug trait impl before Human
    println!("{:#?}", adam); //More usefull for big struct debugging

    //rest operator for Struct
    let imene = Human {
        name: String::from("Imene"),
        age: 29,
        ..adam
    };

    /*
     **Accessing value
     **
     **Value of eyes are borrowed by struct 2 and no longer
     **Available for user 1  because of his type, hungry implement Copy trait
     **because of his type (bool) and is still available
     */
    println!("{}", adam.name);
    println!("{}", adam.hungry);
    //Method call with . related to object
    println!("{:?}", imene.say_hello()); //Impossible for adam to say hello because of borrowed data by imene
    println!("{}", Human::answer()); // Associate function related to classe call with ::

    println!("{}", imene.name);
    println!("{}", imene.hungry);

    /* The type of this 2 struct are different because they're instances
    of different tuple structs */
    let _black = Color(0, 0, 0);
    let _position = Position(-23, 34, 100);
    let _subject = AlwaysEqual;

    // Destructure a tuple struct
    struct Pair(u8, f64);
    let pair = Pair(1, 0.1);
    let Pair(integer, decimal) = pair;
}
