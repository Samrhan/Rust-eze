use std::io::{Read, Write};

mod server;
mod client;
mod command;
// Moved Db type alias to server module
// type Db = HashMap<String, (String, Option<Instant>)>;

fn main() {
    server::start("127.0.0.1:6379");
}