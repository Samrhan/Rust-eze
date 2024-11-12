# Rust Redis Server

## Overview

This project is a simple Redis TCP server implemented in Rust. It supports basic commands like `SET`, `GET`, `ECHO`, and `PING`. The server uses multithreading to handle multiple client connections concurrently, ensuring efficient and responsive communication.

## Why This Implementation?

### Rust

Rust is chosen for its performance, safety, and concurrency features. It provides memory safety without a garbage collector, making it ideal for systems programming and network applications.

### TCP Server

A TCP server is implemented to handle client-server communication over a reliable, connection-oriented protocol. This ensures that data is transmitted accurately and in order.

### Commands

The server supports the following commands:
- `SET`: Stores a key-value pair in the database with an optional expiry time.
- `GET`: Retrieves the value associated with a key.
- `ECHO`: Returns the input message.
- `PING`: Responds with `PONG`.

## Code Structure

- `src/main.rs`: Entry point of the application. Starts the server.
- `src/server.rs`: Contains the server implementation, including the listener and connection handling.
- `src/client.rs`: Handles individual client connections and processes commands.
- `src/command.rs`: Defines supported commands and their handling logic.

## How It Works

1. **Server Initialization**: The server binds to a specified address and starts listening for incoming connections.
2. **Connection Handling**: For each incoming connection, a new thread is spawned to handle the client.
3. **Command Processing**: The client handler reads data from the client, parses the command, and executes the appropriate action.
4. **Database Management**: The server maintains a shared in-memory database using `Arc<Mutex<Db>>`. This allows safe concurrent access and modification of the database.

## Running the Server

To run the server, use the following command:

```sh
cargo run
```