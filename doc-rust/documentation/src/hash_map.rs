use std::collections::HashMap;

fn main() {
    //When you want a keyed collection of values that all have the SAME TYPE,
    //you want a Map type, hashmap stored Key value pair.
    let mut map = HashMap::new();
    map.insert("key1", "value1");
    map.insert("key2", "value2");
    println!("{:?}", map.get("key1"));
    println!("{:?}", map.get("key2"));

    //get method return a borrowed value so in this case "&&str" wrapped in an Option or None
    //unwrap_or need to produce the same type as the Option's type so we pass &""
    println!("{}", map.get("key1").unwrap_or(&""));
    println!("{}", map.get("key2").unwrap_or(&""));

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    //Create a HashMap with a list of teams and a list of score
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    println!("Team list {:#?}", teams);
    println!("Score list: {:#?}", initial_scores);

    //Collect can generate multiple type of data by insering <_, _> rust can deduce
    //The type of data for key and value via the data stored in the vector
    let mut scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();
    println!("HashMap {:#?}", scores);

    //Ownership rules for Hashmap
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and

    //Accessing value in Hashmap
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    //Get return an Option<&V> if there's no value for that key in the hash map,
    //get will return None. The program will need to handle the Option.
    let score = scores.get(&team_name);

    //Iterate trough HasMap
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    //Change value in HashMap
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);

    //Insert value if key does not already have a value
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);

    //Common useCase with Hashmap
    let text = "hello world wonderful world";

    let mut map = HashMap::new();
    //Iterate over sub sclices separate by whitespace
    for word in text.split_whitespace() {
        //or_inser return a mutable ref (&mut V) to the value for the specified key and store that value to count
        let count = map.entry(word).or_insert(0);
        //Dereference count with (*) Increment count
        *count += 1;
    }
    //The mutable reference goes out of scope at the end of the for loop,
    //so all of these changes are safe and allowed by the borrowing rules.

    println!("{:?}", map);
}
