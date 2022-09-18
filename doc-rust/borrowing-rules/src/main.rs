fn borrow_testing() {

    let mut s = String::from("hello");

    let r1 = &s; // OK
    let r2 = &s; // OK
    let r3 = &mut s; // NOT OK
    
    //Cannot use mutable ref in same time as ref as read from other source
    println!("{}, {}, and {}", r1, r2, r3);
}


fn pending() -> &String {
    let s = String::from("hello");

    &s
}

fn main() {
    // let mut s = String::from("hello");
    //
    // let r1 = &s; // OK
    // let r2 = &s; // OK 
    // println!("{} et {}", r1, r2);
    // // r1 and r2 are unused from here and they leave the scope
    // // we can use mutable ref to the same value to altere it
    //  
    // let r3 = &mut s; // sans problème
    // println!("{}", r3);


    let refere_to_nothing = pending();
}


