//! Basic example demonstrating the custom tracing logger
//! 
//! Run with: cargo run --example basic
//! 
//! To see debug logs: $env:RUST_LOG='debug'; cargo run --example basic

use custom_tracing_logger;
use tracing::{debug, error, info, warn};

#[tokio::main]
async fn main() {
    // Initialize the JSON logger
    custom_tracing_logger::init();

    // Log messages at different levels
    info!("Application starting up");
    
    debug!("This is a debug message");
    
    warn!(
        user_id = 12345,
        action = "login_attempt",
        "User attempting to log in"
    );
    
    info!(
        request_id = "req-abc123",
        method = "GET",
        path = "/api/users",
        status = 200,
        duration_ms = 45,
        "HTTP request completed"
    );
    
    error!(
        error_code = "DB_CONNECTION_FAILED",
        retry_count = 3,
        "Database connection failed after retries"
    );

    // Demonstrate structured logging with custom fields
    let user_name = "alice";
    let session_id = "sess-xyz789";
    
    info!(
        user = user_name,
        session = session_id,
        event = "user_action",
        "User performed an action"
    );

    info!("Application shutting down");
}