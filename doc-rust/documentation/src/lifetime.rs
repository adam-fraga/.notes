/*
Lifetimes provide a way to erase dangling references, in c or cpp we use malloc
and free to manualy manage the data in the heap (error prone)

In other langage a garbage
collector manage the heap and don't allow dengling references untill a pointer
still point on a data in heap (lost efficiency).

Rust can staticaly determine if there is dengling reference and does not compile if
this is ambiguous. You have to manualy annote the lifetyme for a given data to help
the compiler to perfectly understand the lifetyme of the given values.
Lifetimes allows us to communicate to the compiler that some references are "related" and 
are expected to share the same lifetime.
*/

//You have to specify a lifetime for all references store on a struct
pub struct Request<'a> {
    path: &'a str,
    query_string: Option<&'a str>,
    method: &'a str,
}

fn main() {}
