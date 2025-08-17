//! Test the convenience macros
//! Run with: cargo run --example test_macros

use custom_tracing_logger::{log_request, log_error};

#[tokio::main]
async fn main() {
    custom_tracing_logger::init();
    
    // Test log_request macro
    log_request!("GET", "/api/users", 200, 45);
    log_request!("POST", "/api/users", 201, 120, user_id = 123);
    
    // Test log_error macro  
    log_error!("DB_CONNECTION_FAILED", "Database timeout");
    log_error!("AUTH_FAILED", "Invalid token", user_id = 123, ip = "192.168.1.1");
}