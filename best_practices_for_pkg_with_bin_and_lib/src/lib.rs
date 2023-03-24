pub fn greeting() {
    println!("Hello, world!");
}

pub mod server;
pub use server::Server;
pub use server::sub_server::Server as SubServer;