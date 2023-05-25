fn main() {
    //Array have a fix size
    let arr: [u8; 5] = [1, 2, 3, 4, 5];
    println!("{arr[0]}");

    /*
     ** Collections contrarly to Tuple, Array, int, char and bool are store to the heap,
     ** when a value store on the heap leave a scope the value is drop
     ** Vectors can only store values that are the same type, for different type use an Enum
     ** combine to a vector.
     */

    //Create a vector
    let mut v: Vec<i32> = Vec::new();
    let v2 = vec![1, 2, 3];

    //Create a vector which cannot grow (reallocate) with size of 10
    let v_with_fixed_capacity: Vec<i32> = Vec::with_capacity(10);

    //Add value to a Vector
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    v.push(9);

    //length of the vector
    v.len();
    //Capacity of the vector (Size available before it realocate)
    v.capacity();
    //Remove the last element from the Array
    v.pop();
    //Remove a value to a specific index
    v.remove(0);

    // We can reference a vector value by 2 way
    let third: &i32 = &v[2]; //store the reference in third
    println!("The third element is {}", third);

    match v.get(2) {
        //use the get method to get the reference
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    let mut v = vec![1, 2, 3, 4, 5];

    /*
     **When the get method is passed an index that is outside the vector, it returns None without panicking.
     **This provide an easy way to display a custom message, rather than leave the programm with error.
     */

    let _does_not_exist = v.get(100);

    let first = &v[0];
    /*
     ** We cannot append a value while we are holding a reference to the first item of the vector (first)
     ** if the memory need to be realocate because of the size of the vector first will point on deallocate memory
     */
    v.push(6);

    println!("The first element is: {}", first);

    //Iterate trought each element of a vector
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    //To change value of a mutable vector you need to iterate trough mutable reference of each val
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
}
