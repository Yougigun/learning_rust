#![allow(
    dead_code,
    unused_imports,
    unused_variables,
    unused_mut,
    unreachable_code,
    clippy::vec_init_then_push,
    clippy::unnecessary_sort_by,
    clippy::match_like_matches_macro,
    clippy::mutable_key_type,
    clippy::single_component_path_imports
)]
use igd;
use std::net::SocketAddrV4;
use std::str::FromStr;

#[tokio::main]
async fn main() {}

// using test function to remove the port
#[tokio::test]
async fn test() {
    let gateway = match igd::search_gateway(Default::default()) {
        Ok(g) => g,
        Err(e) => panic!("Failed to find gateway: {}", e),
    };
    println!("Gateway = {}", gateway);

    // remove port forwarding
    match gateway.remove_port(igd::PortMappingProtocol::TCP, 44019) {
        Err(ref err) => {
            println!("There was an error! {}", err);
        }
        Ok(_) => {
            println!("It worked. port forwarding removed");
        }
    }
}

// using test function to add port
#[tokio::test]
async fn add_port() {
    let gateway = match igd::search_gateway(Default::default()) {
        Ok(g) => g,
        Err(e) => panic!("Failed to find gateway: {}", e),
    };
    println!("Gateway = {}", gateway);

    let local_addr = SocketAddrV4::from_str("192.168.1.46:7878").unwrap();

    // forward port 6902 on router to port 7878 on local machine
    match gateway.add_port(
        igd::PortMappingProtocol::TCP,
        6902,
        local_addr,
        3600,
        "add_port example",
    ) {
        Err(ref err) => {
            println!("There was an error! {}", err);
        }
        Ok(_) => {
            println!(
                "It worked. port forwarded: router's port :{} -> 192.168.1.46:7878",
                6902
            );
        }
    }
}

#[tokio::test]
async fn add_any_port() {
    let gateway = match igd::search_gateway(Default::default()) {
        Ok(g) => g,
        Err(e) => panic!("Failed to find gateway: {}", e),
    };
    println!("Gateway = {}", gateway);

    let local_addr = SocketAddrV4::from_str("192.168.1.46:7878").unwrap();

    // forward port 6902 on router to port 7878 on local machine
    match gateway.add_any_port(
        igd::PortMappingProtocol::TCP,
        local_addr,
        3600,
        "add_port example",
    ) {
        Err(ref err) => {
            println!("There was an error! {}", err);
        }
        Ok(port) => {
            println!(
                "It worked. port forward: router's port {} -> {}",
                port, local_addr
            );
        }
    }
}
