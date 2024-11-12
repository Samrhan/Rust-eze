use std::io::Write;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use crate::server::Db;

pub enum Command {
    SET,
    GET,
    ECHO,
    PING,
    UNKNOWN,
}

impl Command {
    pub fn from_str(command: &str) -> Self {
        match command {
            "SET" => Command::SET,
            "GET" => Command::GET,
            "ECHO" => Command::ECHO,
            "PING" => Command::PING,
            _ => Command::UNKNOWN,
        }
    }
}

pub fn handle_command(
    command: Command,
    parts: &[&str],
    stream: TcpStream,
    db: Arc<Mutex<Db>>,
) {
    match command {
        Command::SET => {
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
                send_response(stream, "+OK\r\n");
            }
        }
        Command::GET => {
            if parts.len() > 4 {
                let key = parts[4].to_string();
                let db = db.lock().unwrap();
                if let Some((value, expiry)) = db.get(&key) {
                    if expiry.is_none() || expiry.unwrap() > Instant::now() {
                        let response = format!("${}\r\n{}\r\n", value.len(), value);
                        send_response(stream, &response);
                    } else {
                        send_response(stream, "$-1\r\n");
                    }
                } else {
                    send_response(stream, "$-1\r\n");
                }
            }
        }
        Command::ECHO => {
            let message = parts[4];
            let response = format!("${}\r\n{}\r\n", message.len(), message);
            send_response(stream, &response);
        }
        Command::PING | Command::UNKNOWN => {
            send_response(stream, "+PONG\r\n");
        }
    }
}

fn send_response(mut stream: TcpStream, response: &str) {
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}