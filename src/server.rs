use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::sync::{Arc, Mutex};
use std::thread;

fn handle_client(mut stream: TcpStream, clients: Arc<Mutex<Vec<TcpStream>>>) {
    let mut buffer = [0; 512];

    loop {
        match stream.read(&mut buffer) {
            Ok(size) => {
                if size == 0 { return; }

                let message = String::from_utf8_lossy(&buffer[..size]);

                let clients = clients.lock().unwrap();
                for client in clients.iter() {
                    let mut client = client.try_clone().unwrap();
                    let _ = client.write_all(message.as_bytes());
                }
            }
            Err(_) => return,
        }
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("Server running on port 7878");

    let clients = Arc::new(Mutex::new(Vec::new()));

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        clients.lock().unwrap().push(stream.try_clone().unwrap());

        let clients_clone = Arc::clone(&clients);

        thread::spawn(move || {
            handle_client(stream, clients_clone);
        });
    }
}
