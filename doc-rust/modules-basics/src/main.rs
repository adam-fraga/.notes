//MOD IN ONE FILE

//Module can be imported to be use even if it's in the same file
use server::Server;

fn main() {
    //Import of struct with use allow us to directly call the Server struct
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run();
}

/*
mod keyword allow you to create module
A module can be private or public
If you want to access the data inside a module nested struct have to be public too
*/
pub mod server {
    pub struct Server {
        addr: String,
    }

    impl Server {
        pub fn new(addr: String) -> Self {
            Server { addr }
        }
        pub fn run(self: Self) {
            println!("Listening on {}", &self.addr);
        }
    }
}

//Module can be nested inside others modules
mod http {
    //By default nested modules are private
    pub mod request {
        use super::method::Method;

        #[derive(Debug)]
        struct Request {
            path: String,
            query: Option<String>,
            //Super keyword allow you to access the parent module
            // method: super::method::Method, (NORMALY)
            method: Method, //use of line 34 allow to directly call Method
        }
    }

    pub mod method {
        #[derive(Debug)]
        pub enum Method {
            GET,
            POST,
            PUT,
            PATCH,
            DELETE,
            HEAD,
            CONECT,
            OPTION,
            TRACE,
        }
    }
}
