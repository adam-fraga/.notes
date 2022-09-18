/* Modules provide a way to organize code and define accessibility of data.
** Modules contains definitions of several data types (Enums, structs, fn, traits const, modules...)
** Modules are not map to files, each "mod" keyword define a module which it can be compare to a file in py or js.
** Each file can have multiple module and nested module.
** Visualize the modules, their datas and accessibility of each data with:
** cargo modules generate tree --with-types
*/
#![allow(dead_code, unused_variables)]

/* The mod keyword allow you to define module */
mod database {
    //pub keyword allow the data to be accessed everywhere
    pub enum Status {
        Connected,
        Interrupt,
    }
    pub fn connect_to_database() -> Status {
        Status::Connected
    }
    pub fn get_user() {
        /*Get user from DB*/
    }
}

mod auth_utils {
    /*
    Credentials here is nested into models modules so we have to specify the rel
    or abs path to Credentials
     */
    pub fn login(creds: models::Credentials) {
        //Relative path
        //authenticate...

        //get_user is private here so we have to change it to public
        crate::database::get_user(); //Absolute path
    }

    fn logout() { /*logout the user*/
    }

    pub mod models {
        pub struct Credentials {
            username: String,
            password: String,
        }
    }
}

/*
**The "pub" keyword turn the accessibility of auth public, Credentials is Private so we have to change it too
**Here we have to turn many data in public and if we declare all the path from each data inside the different Modules
**It can be complicate to understand the code.
** To fix the path declaration of a module we can can use the "use " keyword and the path as an alias
** Use says "whatever where is "
** Here we said whenever Credentials is use in this scope it refere to Credentials from crates::auth_utils::models
** In other terms we just bringing data into the scope
*/
use crate::auth_utils::models::Credentials;
use database::Status; //use keyword allow you to call "Status" in this scope

//Use allow you to call just "Credentials"
pub fn authenticate(creds: Credentials) {
    if let Status::Connected = database::connect_to_database() {
        auth_utils::login(creds);
    }
}
