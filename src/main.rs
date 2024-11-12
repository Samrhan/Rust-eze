use std::io::{Read, Write};
use std::net::TcpListener;
use std::thread;

fn send_response(mut stream: std::net::TcpStream, response: &str) {
    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn handle_client(mut stream: std::net::TcpStream) {
    let mut buffer = [0; 512];

    loop {
        match stream.read(&mut buffer) {
            Ok(0) => break, // Connection closed
            Ok(_) => {
                let request = String::from_utf8_lossy(&buffer);
                let mut parts = request.split("\r\n");

                if let (Some(_), Some(_), Some(command), Some(_), Some(message), ..) = (parts.next(), parts.next(), parts.next(), parts.next(), parts.next()) {
                    if command.eq_ignore_ascii_case("ECHO") {
                        let response = format!("${}\r\n{}\r\n", message.len(), message);
                        send_response(stream.try_clone().unwrap(), &response);
                    } else {
                        send_response(stream.try_clone().unwrap(), "+PONG\r\n");
                    }
                } else {
                    send_response(stream.try_clone().unwrap(), "-ERR Invalid request\r\n");
                }
            }
            Err(e) => {
                println!("Error reading from stream: {}", e);
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
                println!("Accepted new connection");
                thread::spawn(move || {
                    handle_client(stream);
                });
            }
            Err(e) => {
                println!("Error accepting connection: {}", e);
            }
        }
    }
}