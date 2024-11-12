use std::collections::HashMap;
use std::net::TcpListener;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use crate::client::handle_client;

pub type Db = HashMap<String, (String, Option<Instant>)>;

pub struct Config {
    pub dir: String,
    pub dbfilename: String,
}

pub fn start(address: &str, dir: String, dbfilename: String) {
    let listener = TcpListener::bind(address).unwrap();
    let db: Arc<Mutex<Db>> = Arc::new(Mutex::new(HashMap::new()));
    let config = Arc::new(Config { dir, dbfilename });

    // Start a separate thread for key expiry
    let db_clone = Arc::clone(&db);
    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_millis(100));
            let mut db = db_clone.lock().unwrap();
            db.retain(|_, (_, expiry)| expiry.is_none() || expiry.unwrap() > Instant::now());
        }
    });

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("accepted new connection");
                let db = Arc::clone(&db);
                let config = Arc::clone(&config);
                thread::spawn(move || {
                    handle_client(stream, db, config);
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}