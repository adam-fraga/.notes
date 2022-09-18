fn main() {
    let x = 5; // to use shadowing you must reuse the same name with the let keyword (new declaration)

    let x = x + 1; // Shadowing allow x to shadow the first declaration

    // Curly bracket allow us to create a new scope in rust
    {
        let x = x * 2; // in this scope  x value shadow the 2 before
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x); //here x is the second one du to the global scope the previous x is dead

    // Shadowing allow mutable type and allow you to use one name here instead spaces_str and spaces_int
    let spaces = "   ";
    let spaces = spaces.len();
    
    // mut doesnt allow you to mute the variable type 
    let mut spaces2 = "   ";
    spaces2 = spaces2.len();
