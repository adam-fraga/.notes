use rand::Rng;
use std::{cmp::Ordering, io}; //Standard library I/O

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    //loop create an infinite loop
    loop {
        println!("The secret number is: {}", secret_number);

        // :: is an operator to call associate function
        let mut guess = String::new(); //New instance of String

        println!("Please input your guess.");

        //import of std::io allow us to use this instead of std::io::stdin
        io::stdin()
            //readLine return a result Type which is an enum, it contain two variant Ok and Err
            //Result type are oftenly managed with th match keyword as Ordering show it bellow in the code
            .read_line(&mut guess) // Reference as variable are immutable by default
            //If the result of the type is err except method will exit the program and will display the string it take as arg
            //Otherwise if it's Ok expect will return the correct result
            .expect("Failed to read line");

        // convert guess as a uint32 to compare with rand
        // Trim (string method remove space before and after the sequences)
        // parse (conversion method) return an enum of type Result Ok or Err
        // shadowing allow us to redeclare a new guess it shadows the precedent one
        // one case of shadowing is when you want to convert a type to another without redeclare a variable
        let guess: u32 = match guess.trim().parse() {
            //as read_line parse return a Type result, it's a type to facilite the management of errors
            //Again it take 2 variant Ok and Err, Ok contain the result, Err contain the errors
            Ok(num) => num,
            // _ is a master key value that allow us in this case to bypass parses errors and
            Err(_) => continue, // continue go to the next iteration
        };

        println!("You guessed: {}", guess); //format string

        // cmp can compare many thing it take a reference of the variable
        match guess.cmp(&secret_number) {
            //match contains different branch each branch contain a pattern that will execute code mateching to the branch
            //Here Ordering enums are branches belongs to the match expression
            Ordering::Less => println!("Too small!"), //Ordering is an enum
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
