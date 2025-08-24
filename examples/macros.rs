//! Example: Using convenience macros
//! Run with: cargo run --example macros

use custom_tracing_logger::{log_error, log_request};
use tracing::info;

fn main() {
    custom_tracing_logger::init();

    info!("=== Convenience Macros Example ===");

    // HTTP request logging
    log_request!("GET", "/api/users", 200, 45);
    log_request!("POST", "/api/users", 201, 120, user_id = 123);
    log_request!(
        "DELETE",
        "/api/users/456",
        204,
        32,
        user_id = 456,
        admin = true
    );

    // Error logging
    log_error!("DB_CONNECTION_FAILED", "Database timeout");
    log_error!(
        "AUTH_FAILED",
        "Invalid token",
        user_id = 123,
        ip = "192.168.1.1"
    );
    log_error!(
        "RATE_LIMIT_EXCEEDED",
        "Too many requests",
        user_id = 789,
        requests_per_minute = 150
    );

    info!("=== Macros example completed ===");
}
