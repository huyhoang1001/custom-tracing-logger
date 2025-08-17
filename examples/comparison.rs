//! Comparison: Our custom logger vs default tracing
//! 
//! Run with: $env:RUST_LOG='info'; cargo run --example comparison

use tracing::{info, warn};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    println!("=== Using our custom JSON logger ===");
    custom_tracing_logger::init();
    
    info!(user_id = 123, action = "login", "User logged in");
    warn!(error_code = "TIMEOUT", retry = 3, "Request timeout");
    
    println!("\n=== What default tracing looks like ===");
    println!("Default tracing output would be:");
    println!("2024-01-15T10:30:45.123456Z  INFO comparison: User logged in user_id=123 action=login");
    println!("2024-01-15T10:30:45.234567Z  WARN comparison: Request timeout error_code=TIMEOUT retry=3");
}