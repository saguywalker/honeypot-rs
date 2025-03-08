use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:2222").expect("Could not bind to address");

    println!("Honeypot is Listening on port 2222");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    handle_client(stream);
                });
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
}

fn handle_client(mut stream: TcpStream) {
    let peer_addr = match stream.peer_addr() {
        Ok(addr) => addr.to_string(),
        Err(_) => "Unknown".to_string(),
    };

    let mut buffer = [0; 1024];
    let now = get_epoch_time();

    println!("Received connection from {}", peer_addr);

    let filename = format!("/opt/honeypot/{}_{}.log", now, peer_addr);

    let mut file = std::fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(filename)
        .expect("Could not open file");

    loop {
        match stream.read(&mut buffer) {
            Ok(n) => {
                if n == 0 {
                    println!("Connection closed by {}", peer_addr);
                    break;
                }

                let request = String::from_utf8_lossy(&buffer[..n]);
                println!("Received request from {}: {}", peer_addr, request);

                file.write_all(&buffer[..n])
                    .expect("Could not write to file");

                file.sync_all().expect("Could not sync file");
            }
            Err(e) => {
                eprintln!("Error reading from {}: {}", peer_addr, e);
                break;
            }
        }
    }
}

fn get_epoch_time() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs()
}
