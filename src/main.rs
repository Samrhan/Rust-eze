use std::io::{Read, Write};
use std::net::TcpListener;
use std::thread;

fn send_response(mut stream: std::net::TcpStream, response: &str) {
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn handle_client(mut stream: std::net::TcpStream) {
    let mut buffer = [0; 512];
    loop {
        match stream.read(&mut buffer) {
            Ok(0) => break,
            Ok(_) => {
                let request = String::from_utf8_lossy(&buffer);
                let parts: Vec<&str> = request.split("\r\n").collect();
                if parts.len() > 4 && parts[2].eq_ignore_ascii_case("ECHO") {
                    let message = parts[4];
                    let response = format!("${}\r\n{}\r\n", message.len(), message);
                    send_response(stream.try_clone().unwrap(), &response);
                } else {
                    send_response(stream.try_clone().unwrap(), "+PONG\r\n");
                }
            }
            Err(e) => {
                println!("error reading from stream: {}", e);
                break;
            }
        }
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("accepted new connection");
                thread::spawn(move || {
                    handle_client(stream);
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}