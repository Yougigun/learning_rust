#![allow(
    dead_code,
    unused_imports,
    unused_variables,
    unused_mut,
    unreachable_code
)]
// define a server struct
#[derive(Debug, Default)]
pub struct Server {
    pub name: String,
    pub port: u16,
    pub ip: String,
}

// implement a new server function
impl Server {
    pub fn new() -> Server {
        Server {
            name: "Server".to_string(),
            port: 8080,
            ip: "".to_string(),
        }
    }
}

pub mod sub_server;
