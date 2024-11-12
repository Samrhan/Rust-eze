use std::env;

mod server;
mod client;
mod command;

fn main() {
    let args: Vec<String> = env::args().collect();
    let dir = args.iter().position(|r| r == "--dir").map(|i| args[i + 1].clone()).unwrap_or_else(|| "/tmp/redis-data".to_string());
    let dbfilename = args.iter().position(|r| r == "--dbfilename").map(|i| args[i + 1].clone()).unwrap_or_else(|| "dump.rdb".to_string());

    server::start("127.0.0.1:6379", dir, dbfilename);
}