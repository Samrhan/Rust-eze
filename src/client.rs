use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::{Arc, Mutex};

use crate::command::{handle_command, Command};
use crate::server::Db;

fn send_response(mut stream: TcpStream, response: &str) {
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

pub fn handle_client(mut stream: TcpStream, db: Arc<Mutex<Db>>) {
    let mut buffer = [0; 512];
    loop {
        match stream.read(&mut buffer) {
            Ok(0) => break,
            Ok(_) => {
                let request = String::from_utf8_lossy(&buffer);
                let parts: Vec<&str> = request.split("\r\n").collect();
                if parts.len() > 4 {
                    let command = parts[2].to_uppercase();
                    let command = Command::from_str(&command);
                    handle_command(command, &parts, stream.try_clone().unwrap(), db.clone());
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