use std::net::UdpSocket;
use std::str;

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:8080")?;

    loop {
        let mut buf = [0; 1024];
        let (amt, src) = socket.recv_from(&mut buf)?;

        let received_data = str::from_utf8(&buf[..amt]).expect("Invalid UTF-8 data received");
        println!("Received '{}' from {}", received_data, src);

        socket.send_to(&buf[..amt], &src)?;
    }
}
