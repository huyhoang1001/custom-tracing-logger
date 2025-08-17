//! Example showing file-only logging (no console output)
//! 
//! Run with: $env:RUST_LOG='info'; $env:LOG_FILE_DIR='./logs'; $env:LOG_FILE_PREFIX='silent'; $env:LOG_FILE_ONLY='true'; cargo run --example file_only

use custom_tracing_logger;
use tracing::{info, warn, debug};

#[tokio::main]
async fn main() {
    // Single init - LOG_FILE_ONLY=true disables console output
    custom_tracing_logger::init();

    println!("Regular println still shows on console");
    
    info!("This goes only to file, not console");

    debug!("DEBUG -This goes only to file, not console");
    warn!(user_id = 456, "This warning is silent on console");
    
    println!("Check ./logs/silent.YYYY-MM-DD for the logs");
}