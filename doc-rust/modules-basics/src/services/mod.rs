// MOD SPLITED FILES

//Mod file is used for rust to understand that http folder is the root
//of our module method.rs & mod.rs. here we define the visibility of
//the files in http folder and import them to be used outside.
pub use serviceClient::Client;
pub use serviceServer::Server;

pub mod serviceClient;
pub mod serviceServer;
