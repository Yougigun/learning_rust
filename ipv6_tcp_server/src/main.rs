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
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn handle_client(mut stream: TcpStream) {
    let mut data: [u8; 50] = [0; 50]; // using 50 byte buffer
    while match stream.read(&mut data) {
        Ok(size) => {
            if size == 0 {
                // sleep for 1 second
                std::thread::sleep(std::time::Duration::from_millis(100));

                true
            } else {
                println!("data: {:?}", &data[0..size]);
                stream.write_all(&data[0..size]).unwrap();
                true
            }
            // echo everything!
        }
        Err(x) => {
            println!("error :{:?}", x);
            println!(
                "An error occurred, terminating connection with {}",
                stream.peer_addr().unwrap()
            );
            stream.shutdown(std::net::Shutdown::Both).unwrap();
            false
        }
    } {}
}

fn main() {
    let listener = TcpListener::bind("[::1]:3333").unwrap();
    // accept connections and process them, spawning a new thread for each one
    println!("Server listening on port 3333");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move || {
                    // connection succeeded
                    handle_client(stream)
                });
            }
            Err(e) => {
                // connection failed
                println!("Error: {}", e);
            }
        }
    }

    // close the socket server
    drop(listener);
}
// test client
#[test]
fn test_client() {
    use std::str::from_utf8;
    match TcpStream::connect("[::1]:3333") {
        Ok(mut stream) => {
            println!("Successfully connected to server in port 3333");

            let msg = b"Hello!";

            stream.write_all(msg).unwrap();
            println!("Sent Hello, awaiting reply...");

            let mut data: [u8; 50] = [0; 50]; // using 50 byte buffer
            match stream.read(&mut data) {
                Ok(s) => {
                    if let Ok(text) = from_utf8(&data[..s]) {
                        println!("Received reply: {}", text);
                        assert_eq!(text, "Hello!"); // Replace "Expected reply" with the actual expected reply
                    }
                }
                Err(e) => {
                    panic!("Failed to receive data: {}", e);
                }
            }
        }
        Err(e) => {
            panic!("Failed to connect: {}", e);
        }
    }
}
