fn main() {
    //Slice in rust are part of string and are immutables, it's useful and cheap to use them when you have
    //To parse data but not modify them.

    let s = String::from("Hello world");

    first_word(&s);
    first_word_right_way(&s);
    slice_are_awesomes();

    let s = String::from("127.0.0.1:8080");
    let s_slice = &s[10..]; //Syntax to display part of slice here from the 10th byte to the last

    /*
    (Warning number up there "[10..]" in this syntax refere to byte wich is not a good way to manage slices)
    some character as emoji or foreigners alphabet are not UTF-8 encoded (1byte/character)
    like japanese, cyrilic or even emoji try to retrieve that info with that part slice rust compiler will throw an error
    */

    //Slice can be a litteral or a reference to a string
    let s_borrow: &str = &s;
    let s_litteral = "1234"; // <-------Litteral in rust are slices

    dbg!(&s);
    dbg!(s_slice);
    dbg!(s_borrow);
    dbg!(s_litteral);

    // Function that return the index of the word to finaly get the first word (constraint)
    fn first_word(s: &String) -> usize {
        let bytes = s.as_bytes(); //as_byte convert string to a table of bytes (octets)
        for (i, &element) in bytes.iter().enumerate() {
            //Enumerate return a tuple of (index, item) -> which are destructured here to "i" and "&element".
            if element == b' ' {
                //Byte litreral syntax ( If the byte is equel to space then return the index of the word.
                return i;
            }
        }
        s.len()
    }

    fn first_word_right_way(s: &str) -> &str {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i]; //Return a slice which is a reference to s
                                 //from begin to index which equals to "space"
            }
        }
        &s[..] //if no space found return reference to the entire string
    }

    //Prefere to use Slice as python Rust provide slices it work on string

    fn slice_are_awesomes() {
        let s = String::from("hello world");

        let size = s.len();

        let hello = &s[0..5];
        let world = &s[6..11];
        let slice = &s[..2];
        let slice = &s[3..size];
        let slice = &s[..];

        println!("{}", hello);
        println!("{}", world);
        println!("{}", slice);
    }

    //STRING

    //Turn &str to String with .to_owned() (Create owned data by borrow data usualy by cloning)
    let my_real_string = "string literal!".to_owned();
    let normal_string = String::from("Hello world");
    print_type_of(&my_real_string);
    print_type_of(&normal_string);

    //Method to create string

    //Implemented by default in Display trait if value change for some reason it can throm an error
    let my_str1 = "Hello".to_string();
    //Use to turn something into String habitualy for casting &str to String
    let my_str2: String = "Hello".into();
    //Only for format !
    let my_str3 = format!("Hello");

    print_type_of(&my_str1);
    print_type_of(&my_str2);
    print_type_of(&my_str3);

    fn print_type_of<T>(_: &T) {
        println!("Type is: {}", std::any::type_name::<T>())
    }

    //PARSE STRING

    let sentence = String::from("Here is the sentences");

    //chars method return an iterator of character
    let mut iter_char = sentence.chars();

    //bytes method return an iterator of character
    let mut iter_byte = sentence.bytes();

    loop {
        //Next give us the next element in the iterator
        let item = iter_char.next();
        match item {
            Some(c) => {
                println!("Char: {}", c)
            }
            None => break,
        }
    }
    /*
    Enumerate method provide a way to obteain a key value pair in a
    tuple it generates a new iterator. (Here we use destructuration)
    */
    for (i, c) in sentence.chars().enumerate() {
        println!("Index: {} Value: {}", i, c);
        /*
        &str[..i] ->Fetch  All character between index 0 to i
        &str[i + 1..] -> Fetch All character after index i + 1 to skip the space
        i + 1 = +1 byte which is normaly unsafe but here we arre sure " " is ecoding in one byte
        So this code is safe.
        */
        if c == ' ' {
            Some((&sentence[..i], &sentence[i + 1..]));
        }
    }
}
