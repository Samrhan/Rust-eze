use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::sync::{Arc, Mutex};
use std::thread;

fn send_response(mut stream: std::net::TcpStream, response: &str) {
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn handle_client(mut stream: std::net::TcpStream, db: Arc<Mutex<HashMap<String, String>>>) {
    let mut buffer = [0; 512];
    loop {
        match stream.read(&mut buffer) {
            Ok(0) => break,
            Ok(_) => {
                let request = String::from_utf8_lossy(&buffer);
                let parts: Vec<&str> = request.split("\r\n").collect();
                if parts.len() > 4 {
                    let command = parts[2].to_uppercase();
                    match command.as_str() {
                        "SET" => {
                            if parts.len() > 6 {
                                let key = parts[4].to_string();
                                let value = parts[6].to_string();
                                let mut db = db.lock().unwrap();
                                db.insert(key, value);
                                send_response(stream.try_clone().unwrap(), "+OK\r\n");
                            }
                        }
                        "GET" => {
                            if parts.len() > 4 {
                                let key = parts[4].to_string();
                                let db = db.lock().unwrap();
                                if let Some(value) = db.get(&key) {
                                    let response = format!("${}\r\n{}\r\n", value.len(), value);
                                    send_response(stream.try_clone().unwrap(), &response);
                                } else {
                                    send_response(stream.try_clone().unwrap(), "$-1\r\n");
                                }
                            }
                        }
                        _ => {
                            send_response(stream.try_clone().unwrap(), "+PONG\r\n");
                        }
                    }
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
    let db = Arc::new(Mutex::new(HashMap::new()));

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("accepted new connection");
                let db = Arc::clone(&db);
                thread::spawn(move || {
                    handle_client(stream, db);
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}