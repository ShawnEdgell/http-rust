// src/main.rs
use axum::{
    routing::get,
    Router,
    response::{Html, IntoResponse},
    http::StatusCode,
    Json,
};
use std::net::SocketAddr;
use tracing::{instrument, info, warn, error, debug};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

// Our application state (if we had any, e.g., a database pool)
// struct AppState {} // Example

#[tokio::main]
async fn main() {
    // --- Structured Logging Setup (tracing) ---
    // Initialize tracing_subscriber to listen for tracing events
    // You can customize the format and filtering.
    // "env_filter" allows you to control verbosity with RUST_LOG env var.
    // Example: RUST_LOG=rust_hello_server=debug,axum=trace cargo run
    // Fallback to "info" level if RUST_LOG is not set.
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info")); // Default to INFO level for all modules

    // Configure a JSON formatter for logs.
    let formatting_layer = fmt::layer().json().with_target(true).with_thread_ids(true);

    tracing_subscriber::registry()
        .with(env_filter)
        .with(formatting_layer) // Use JSON output
        .init();


    info!("Rust HTTP server starting..."); // This will be a JSON log

    // --- HTTP Server Setup (Axum) ---
    // Define our application routes
    let app = Router::new()
        .route("/", get(root_handler))
        .route("/hello", get(hello_handler))
        .fallback(handler_404); // Fallback for routes not matched

    // Add any state to your app if needed (e.g., database connection pool)
    // .with_state(Arc::new(AppState {}))

    let port = 3000;
    let addr = SocketAddr::from(([0, 0, 0, 0], port)); // Listen on all interfaces
    info!(message = "Server listening", %addr); // Use key-value for structured log fields

    // Run the server
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap_or_else(|e| {
        error!(error = %e, "Failed to bind to address");
        std::process::exit(1);
    });

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap_or_else(|e| {
            error!(error = %e, "Server error");
            // Optionally, you could add more robust error handling for server shutdown here.
        });
}

// Handler for the root path "/"
#[instrument] // Automatically logs entry/exit and arguments of this function at TRACE level
async fn root_handler() -> Html<&'static str> {
    debug!("Serving root path response");
    Html("<h1>Welcome to the Rust HTTP Server!</h1><p><a href=\"/hello\">Say Hello</a></p>")
}

// Handler for the "/hello" path
#[instrument]
async fn hello_handler() -> impl IntoResponse {
    info!("Serving hello world response");
    // You can return various types that implement IntoResponse
    // For simple strings or HTML:
    // Html("Hello, World from Rust!")
    // For JSON:
    Json(serde_json::json!({ "message": "Hello, World from Rust!" }))
}

// Fallback handler for 404 Not Found
#[instrument]
async fn handler_404() -> impl IntoResponse {
    warn!("Resource not found");
    (StatusCode::NOT_FOUND, Html("<h2>404 Not Found</h2><p>Sorry, the page you are looking for does not exist.</p>"))
}

// Basic error handling:
// Axum handlers can return Result<T, E> where E implements IntoResponse.
// This allows you to return proper HTTP error responses.
// For example:
/*
async fn an_operation_that_might_fail() -> Result<String, AppError> {
    // ... some logic
    if something_bad_happened {
        return Err(AppError::InternalServerError("Something went wrong".to_string()));
    }
    Ok("Success".to_string())
}

enum AppError {
    InternalServerError(String),
    NotFound(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            AppError::InternalServerError(msg) => {
                error!(error = %msg, "Internal server error occurred"); // Log the detailed error
                (StatusCode::INTERNAL_SERVER_ERROR, format!("Internal Server Error: {}", msg))
            }
            AppError::NotFound(msg) => {
                warn!(resource_not_found = %msg);
                (StatusCode::NOT_FOUND, format!("Not Found: {}", msg))
            }
        };
        (status, Json(serde_json::json!({ "error": error_message }))).into_response()
    }
}
*/