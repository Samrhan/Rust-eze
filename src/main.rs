use std::io::{Read, Write};
use std::net::TcpListener;

fn main() {

    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("accepted new connection");
                let mut buffer = [0; 512];
                match stream.read(&mut buffer) {
                    Ok(_) => {
                        let response = "+PONG\r\n";
                        stream.write(response.as_bytes()).unwrap();
                        stream.flush().unwrap();
                    }
                    Err(e) => {
                        println!("error reading from stream: {}", e);
                    }
                }
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
    
}
