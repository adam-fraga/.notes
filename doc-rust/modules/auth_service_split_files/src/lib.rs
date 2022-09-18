/*
     Modules provide a way to organize code and define accessibility of data.
     Modules contains definitions of several data types (Enums, structs, fn, traits const, modules...)
     Modules are not map to files, each "mod" keyword define a module which it can be compare
     to a file in py or js.
     Each file can have multiple module and nested module.
     Visualize the modules, their datas and accessibility of each data with:
     cargo modules generate tree --with-types
*/
#![allow(dead_code, unused_variables)]
/*
    If a module has no submodule nested inside of it you can export it has a new file
    and import the module like this even if database.rs is in the same directory rust
    module are not mapped to file system so we have to explicitly declare we want to import
    the module database with this syntax and tell to rust crate have a submodule call database.rs
    Rust behavior :
        1 -> There is no module describe inline (in curly braces)
        2 -> I m going to look at a file called database.rs
        3 -> There is a file called database.rs here is the module !
*/
mod database; //FIle which are module in rust to be import like this (import database content code)

/*
    Here auth_utils have a child module, here is the behavior this time:
        1 -> There is no module describe inline,
        2 -> There is a file call auth_utils here is the module !
*/
mod auth_utils;

/*
    The "pub" keyword turn the accessibility of auth public,
    Credentials is Private so we have to change it too
    Here we have to turn many data in public and if we declare all the path 
    from each data inside the different Modules It can be complicate to understand the code.
    To fix the path declaration of a module we can can use the "use " keyword and the path as an alias
    Use says "whatever where is" Here we said whenever Credentials is use in this scope 
    it refere to Credentials from crates::auth_utils::models In other terms we just bringing 
    data into the scope
*/

pub use crate::auth_utils::models::Credentials; //here we use the pub keyword to reexport Credentials from the top module
use database::Status;

//Use allow you to call just "Credentials"
pub fn authenticate(creds: Credentials) {
    //Use allow you to call "Status""
    if let Status::Connected = database::connect_to_database() {
        auth_utils::login(creds);
    }
}
