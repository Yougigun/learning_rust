use std::net::UdpSocket;

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:8081")?;
    socket.connect("127.0.0.1:8080")?;

    let message = b"Hello, UDP server!";
    socket.send(message)?;

    let mut buf = [0; 1024];
    let amt = socket.recv(&mut buf)?;
    let received_data = std::str::from_utf8(&buf[..amt]).expect("Invalid UTF-8 data received");
    println!("Received '{}' from server", received_data);

    Ok(())
}
