//! Simple usage example showing basic logging
//! Run with: cargo run --example simple_usage

use custom_tracing_logger;
use tracing::{info, warn, error, debug};

fn main() {
    // Initialize the logger - that's it!
    custom_tracing_logger::init();
    
    // Basic logging
    info!("Application started");
    debug!("This debug message shows with RUST_LOG=debug");
    
    // Structured logging with fields
    info!(
        user_id = 123,
        action = "login",
        ip = "192.168.1.1",
        "User logged in successfully"
    );
    
    warn!(
        retry_count = 3,
        timeout_ms = 5000,
        "Operation will be retried"
    );
    
    error!(
        error_code = "DB_CONNECTION_FAILED",
        database = "users_db",
        "Failed to connect to database"
    );
    
    info!("Application finished");
}