/*
    Generics are a way as polymorphism to factorise code, it provide a solution to avoid
    multiplication of data struct
*/
/*
    By convention in Rust Generic are Uppercase and one letter T, U, V, W...
    We declare generic right after the function name by using angle braces, and by
    replacing the datatype by the generics we need here T.
    We use PartialOrd Trait to allow comparisons operation for our generics data
    We use Copy Trait to allow the behavior of copy data for our generics data (int, list, slice...)
*/
fn largest<T: std::cmp::PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    //To allow the moves of a standart data with Copy trait we need to specify the Copy Trait
    for &item in list {
        // Here the compiler throw an error because by default it does'nt now how to compare generics

        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    //We can now use our function for multiple data like for this example chars and integer
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);

    /*
        Generics with struct:
        You can use multiple generic if many data have to be different in struct
    */
    struct Point<T, U> {
        x: T,
        y: U,
    }

    /*
        The first and second exemple work with only one generics, because both x and y belongs
        to the same data type but in the third case we have 2 different type which obliges us
        to use 2 differents generics
    */
    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };

    /*
        Generics with Enum:
        Option and result are both Enum implemented by default and using generics.
    */
    enum Option<T> {
        Some(T),
        None,
    }

    /*
    When you recognize situations in your code with multiple struct or enum definitions
    that differ only in the types of the values they hold, you can avoid duplication
    by using generic types instead.
    */

    enum Result<T, E> {
        Ok(T),
        Err(E),
    }

    //Generics on methods:

    struct Point2<T> {
        x: T,
        y: T,
    }

    /*
        Impl must have <T> to precise to the compiler that the Type in Point2 is generic
        and llow the method to be implemented and functionnal for all generics
    */
    impl<T> Point2<T> {
        fn x(&self) -> &T {
            &self.x
        }
    }
    //Method with type contraint (without generic)
    impl Point2<f32> {
        fn distance_depuis_lorigine(&self) -> f32 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }

    let p = Point2 { x: 5, y: 10 };

    println!("p.x = {}", p.x());
}
