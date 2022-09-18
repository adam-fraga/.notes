/*
Every file in rust is a module, by default you can imagine
your file is wrapped with mod module-name {}
*/

pub enum Status {
    Connected,
    Interrupt,
}
pub fn connect_to_database() -> Status {
    Status::Connected
}
pub fn get_user() { /*Get user from DB*/
}
