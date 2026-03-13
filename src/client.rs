use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::thread;

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:7878")
        .expect("Failed to connect");

    println!("Connected to chat server!");

    let mut read_stream = stream.try_clone().unwrap();

    thread::spawn(move || {
        let mut buffer = [0; 512];
        loop {
            let size = read_stream.read(&mut buffer).unwrap();
            let message = String::from_utf8_lossy(&buffer[..size]);
            println!("{}", message);
        }
    });

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        stream.write_all(input.as_bytes()).unwrap();
    }
}
