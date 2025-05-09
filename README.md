# Rust Hello World HTTP Server

A simple "Hello, World!" HTTP server written in Rust using the Axum web framework. This project demonstrates:

- Basic HTTP server setup with Axum on the Tokio runtime.
- A `/hello` route (returning JSON) and a root `/` route.
- Structured JSON logging using the `tracing` and `tracing-subscriber` crates.

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (via `rustup`) installed.

## Running the Server

1.  Clone or navigate to the project directory:

    ```bash
    # Example:
    # git clone <your-repo-url>
    # cd rust-hello-server
    ```

2.  Build and run the server:
    ```bash
    cargo run
    ```
    The server will compile and then start on `http://localhost:3000`. You will see JSON logs in your terminal.
    To run in release (optimized) mode: `cargo run --release`

## Endpoints

- `GET /`: Displays a welcome HTML message.
  - Example: `curl http://localhost:3000/`
- `GET /hello`: Responds with a JSON message: `{"message":"Hello, World from Rust!"}`.
  - Example: `curl http://localhost:3000/hello`

## Project Structure

- `src/main.rs`: Contains all the server logic, including request handlers and server setup.
- `Cargo.toml`: Manages project dependencies and metadata.

---
