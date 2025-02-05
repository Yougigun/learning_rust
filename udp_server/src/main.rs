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
    clippy::single_component_path_imports,
    clippy::match_single_binding,
    clippy::needless_borrow,
    clippy::missing_safety_doc,
    unused_unsafe
)]
use std::net::UdpSocket;
use std::str;
// what is udp?
// UDP is a connectionless protocol. It is known as a datagram protocol because it is analogous to a
// postal service: when the application layer (layer 7 in the OSI model) passes messages to the
// transport layer (layer 4), they are passed to the UDP. The UDP encapsulates that data in a UDP
// datagram. One significant difference between UDP and TCP is that UDP is a connectionless
// protocol. This means that UDP does not establish a connection before sending data. Instead, UDP
// just sends the data without checking to see whether the recipient is ready to receive the data.
// If the recipient is not ready to receive the data, the data is lost. This means that UDP is
// unreliable. However, UDP is faster than TCP because it does not establish a connection before
// sending data. UDP is used in applications where speed is more important than reliability. For
// example, UDP is used in applications such as video streaming, where lost packets don’t matter.
// UDP is also used in applications such as DNS, where the overhead of TCP is not necessary.

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:8080")?;

    loop {
        let mut buf = [0; 1024];
        let (amt, src) = socket.recv_from(&mut buf)?;

        let received_data = str::from_utf8(&buf[..amt]).expect("Invalid UTF-8 data received");
        println!("Received '{}' from {}", received_data, src);

        socket.send_to(&buf[..amt], src)?;
        println!("Sent '{}' to {}", received_data, src)
    }
    #[derive(Debug)]
    enum Message {
        Quit,
        ChangeColor(i32, i32, i32),
        Move { x: i32, y: i32 },
        Write(String),
    }
    let m = Message::Write(String::from("hello"));
    let a = m;
    println!("{:?}", m);
    let a = vec![1,2];
    let v = a;
    println!("{:?}", a);

}
