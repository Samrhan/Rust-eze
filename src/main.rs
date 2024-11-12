use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

type Db = HashMap<String, (String, Option<Instant>)>;

fn send_response(mut stream: std::net::TcpStream, response: &str) {
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn handle_client(mut stream: std::net::TcpStream, db: Arc<Mutex<Db>>) {
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
                                let mut expiry = None;
                                if parts.len() > 8 && parts[8].to_uppercase() == "PX" {
                                    if let Ok(ms) = parts[10].parse::<u64>() {
                                        expiry = Some(Instant::now() + Duration::from_millis(ms));
                                    }
                                }
                                let mut db = db.lock().unwrap();
                                db.insert(key, (value, expiry));
                                send_response(stream.try_clone().unwrap(), "+OK\r\n");
                            }
                        }
                        "GET" => {
                            if parts.len() > 4 {
                                let key = parts[4].to_string();
                                let db = db.lock().unwrap();
                                if let Some((value, expiry)) = db.get(&key) {
                                    if expiry.is_none() || expiry.unwrap() > Instant::now() {
                                        let response = format!("${}\r\n{}\r\n", value.len(), value);
                                        send_response(stream.try_clone().unwrap(), &response);
                                    } else {
                                        send_response(stream.try_clone().unwrap(), "$-1\r\n");
                                    }
                                } else {
                                    send_response(stream.try_clone().unwrap(), "$-1\r\n");
                                }
                            }
                        }
                        "ECHO" => {
                            let message = parts[4];
                            let response = format!("${}\r\n{}\r\n", message.len(), message);
                            send_response(stream.try_clone().unwrap(), &response);
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
    let db: Arc<Mutex<HashMap<String, (String, Option<Instant>)>>> = Arc::new(Mutex::new(HashMap::new()));
    let db_clone = Arc::clone(&db);
    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_millis(100));
            let mut db = db_clone.lock().unwrap();
            let now = Instant::now();
            db.retain(|_, (_, expiry)| expiry.is_none() || expiry.unwrap() > now);
        }
    });

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