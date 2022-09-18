fn main() {
    // Create a scope
    {
        // String is a namespace it allow mutable string and provide plain of methods to manage them (store on heap)
        let mut s = String::from("hello");

        s.push_str(", world!"); // push_str() appends a literal to a String

        println!("{}", s); // This will print `hello, world!`
    };
    //Rust automaticaly call drop() and free the memory on the heap when the variable String exit the scope

    let s1 = String::from("hello");
    let s2 = s1;

    println!("{}, world!", s1); // Error s1 is borrowed here s2 is the ownership of s1

    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("{}, world!", s1); // Ok but clone provide a more heavy code and is not recommanded

    let s = String::from("hello"); // s enter to the scope

    take_ownership(s); // s is moved to the function and leave the global scope

    println!("{}", s); // Error s moved to take_ownership() and is no longer available here

    let x = 5; // x enter to the scope

    create_copy(x); // x move to the function but can exist to the global scope cause it's a copy

    println!("{}", x); // x is passed by copy and is still available here
}
// x leave the scope and disappear from the stack,
// s has already move into the function so nothing append for s

fn take_ownership(text: String) {
    // text enter to the scope
    println!("{}", text);
} // text leave the scope and drop() is called to free the heap

fn create_copy(integer: i32) {
    // integer enter to the scope
    println!("{}", integer);
} // integer leave the scope and disapear from the stack
