# Rust Hello World HTTP Server (Axum & Dockerized)

A simple "Hello, World!" HTTP server written in Rust using the Axum framework, containerized with Docker, and deployed. This project demonstrates:

- Basic HTTP server setup with Axum on the Tokio runtime.
- A `/` route and a `/hello` route (returning JSON).
- Structured JSON logging using the `tracing` and `tracing-subscriber` crates.
- Containerization with Docker for consistent deployment.
- Serving via a reverse proxy (Caddy) with automatic HTTPS.

## Key Technologies

- Rust (Edition 2024/2021, with Axum, Tokio, Tracing, Serde)
- Docker & Docker Compose
- Caddy (as the reverse proxy on the host VPS)

## Deployed Application

The application is currently hosted on a Virtual Private Server (VPS) and is accessible at:

➡️ **[https://rust.skatebit.app](https://rust.skatebit.app)**

### Endpoints (Live)

- `GET https://rust.skatebit.app/`
  - Displays: Welcome HTML message.
- `GET https://rust.skatebit.app/hello`
  - Responds with JSON: `{"message":"Hello, World from Rust!"}`

## Running for Local Development (Using Docker)

To run this application locally for development or testing using Docker:

### Prerequisites (Local Development)

- [Docker Desktop](https://www.docker.com/products/docker-desktop/) (for Windows/macOS) or Docker Engine (for Linux) installed.
- Git (for cloning the repository).
- Rust toolchain (for local non-Docker builds or if you modify source).

### Steps

1.  **Clone the repository (if you haven't already):**

    ```bash
    git clone [https://github.com/ShawnEdgell/http-rust.git](https://github.com/ShawnEdgell/http-rust.git)
    cd http-rust
    ```

2.  **Build the Docker image:**
    From the project's root directory (where the `Dockerfile` is):

    ```bash
    docker build -t http-rust-app-local .
    ```

3.  **Run the Docker container:**
    This command maps port 3001 on your local machine to port 3000 in the container (as the Rust app listens on 3000).

    ```bash
    docker run -d -p 3001:3000 --name my-rust-test-container http-rust-app-local
    ```

4.  **Access locally:**
    Open your browser and go to:

    - `http://localhost:3001`
    - `http://localhost:3001/hello`

5.  **To stop and remove the local test container:**
    ```bash
    docker stop my-rust-test-container
    docker rm my-rust-test-container
    ```

## Deployment Overview (on VPS)

This application is deployed on a Virtual Private Server (VPS) using:

- **Docker:** The Rust application is containerized using the provided `Dockerfile`.
- **Docker Compose:** The `docker-compose.yml` file on the VPS is used to manage the application container's lifecycle.
- **Caddy:** Acts as a reverse proxy on the VPS, routing traffic from `https://rust.skatebit.app` to the running Docker container and automatically handling SSL/TLS.

## Project Structure

- `src/main.rs`: Main application code with Axum server setup and route handlers.
- `Cargo.toml`: Defines project metadata and Rust dependencies.
- `Cargo.lock`: Ensures reproducible builds by locking dependency versions.
- `Dockerfile`: Instructions to build the Docker image for the application.
- `.dockerignore`: Specifies files to exclude from the Docker build context.
- `README.md`: This file.

---
