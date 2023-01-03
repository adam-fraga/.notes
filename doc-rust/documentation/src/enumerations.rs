/*
*Enums are a special type which have a final set of values, as an example if you want to store
*specific values inside a type of data wich have only a limited knowed value you should use enum
*Example for Ip address which of only 2 type of ip address which are ipv4 & ipv6.
*/
//Basic enum implements no value

#[derive(Debug)]
enum IpAddrKind {
    V4, //By default in memory variant contain number start to 0 and increment
    V6, //= 1
}

//Enum implements value
#[derive(Debug)]
enum IpAddrKindWithData {
    V4(u8, u8, u8, u8), //Variant can contain one or multiples Data
    V6(String),
}

//Enum version (All variant have the same type which is Message)
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

//Struct version (Problem is with function each struct as their own type)
struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct

//As struct enum can implement method
impl Message {
    fn call(&self) {
        println!("Method call:\n {:#?}", &self);
    }
}

fn main() {
    let _four = IpAddrKind::V4; //Variant are namespaced under it's identifier
    let _six = IpAddrKind::V6; //and must be call with :: operator.

    // Enum variand are the same type in this case IpAddrKind (easy to manage with function)
    fn route(ip_kind: IpAddrKind) {
        println!("Type of IP: {:?}", ip_kind);
    }
    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    //Enum can contains data of multiple type (str, struct, bool...)
    //Here the return of the function V4 and V6 return an instance of the type IpAddrKindWithData.
    let _home = IpAddrKindWithData::V4(127, 0, 0, 1);
    let _loopback = IpAddrKindWithData::V6(String::from("::1"));

    //Here self value in call method refere to Write variant (m)
    let m = Message::Write(String::from("hello"));
    m.call();

    //You cannot calcul between an Opt<T> and an other type you must convert Opt<T>
    let some_number = Some(5);
    println!("{:?}", some_number);
    let some_string = Some("a string");
    println!("{:?}", some_string);

    let absent_number: Option<i32> = None;
    println!("{:?}", absent_number);
}
/*
Rust does not implement the concept of null, in place there is Opttion enum,
option enum are implemented in default scope it can check if a value exists or not
<T> here specify a generics type wich is (any type). You can unwrap() an option if the value inside exist.

(Deinition of the option Enum)
enum Option<T> {
    None,
    Some(T),
}
*/
