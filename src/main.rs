use std::io::{Read, Write};
use std::net::TcpListener;
use std::thread;

fn send_pong_response(mut stream: std::net::TcpStream) {
    let response = "+PONG\r\n";
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn handle_client(mut stream: std::net::TcpStream) {
    let mut buffer = [0; 512];
    loop {
        match stream.read(&mut buffer) {
            Ok(0) => break, 
            Ok(_) => {
                send_pong_response(stream.try_clone().unwrap());
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