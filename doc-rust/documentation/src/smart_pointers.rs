/*
* Type or mart pointer:
*
* 1) Rc<T>: This pointer enables you to allow data to have multiple
* owners by keeping track of the number of owners and, when no owners remain, cleaning up the data.
* 2) Box<T>: For allocating values on the heap
* 3) Ref<T> and RefMut<T>, accessed through RefCell<T>, a type that enforces the borrowing rules
* at runtime instead of compile time
*/

//When the compiler try to calcul this datatype it encounter recursion and is not able to
//determine the size of List to fix that problem we use the Box Size.
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}
