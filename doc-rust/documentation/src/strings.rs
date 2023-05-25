fn main() {
    //By default string in Rust are UTF8 encoded

    //Create new string
    let mut s = String::new(); //Empty string
    let s = "Hello world".to_string();
    let s = String::from("Hello world");
    let s = format!("Hello world");

    //Grow a string (String have to be mutable)
    s.push_str("bar");
    let s2 = "loo";
    s.push_str(s2);
    println!("s is {}", s);

    //s2 is borrowed here but push_str() does not take the ownership of s2
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);

    //Push one char
    let mut s = String::from("lo");
    s.push('l');

    // Concatenation
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);

    //Iterate trough char in string
    for c in "Hello world".chars() {
        println!("{}", c);
    }

    for c in "Hello world".bytes() {
        println!("{}", c);
    }

    //REFERE TO SLICE IF YOU NEED TO MANIPULE STRING
}
