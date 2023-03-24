use lib::Server;
use lib::SubServer;
fn main() {
    let server = Server::new();
    // print server name
    println!("Server name: {}", server.name);

    // new sub server
    let sub_server = SubServer::new();
    // print sub server name
    println!("Sub server name: {}", sub_server.name);
}
