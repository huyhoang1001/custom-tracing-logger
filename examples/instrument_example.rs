//! Example showing how #[instrument] works
//! Run with: cargo run --example instrument_example

use custom_tracing_logger;
use tracing::{info, instrument};

#[instrument]
fn process_user(user_id: u64, name: &str) -> String {
    info!("Processing user data");
    
    // Simulate some work
    std::thread::sleep(std::time::Duration::from_millis(10));
    
    format!("Processed user: {} (ID: {})", name, user_id)
}

#[instrument]
fn parent_task() {
    info!("Starting parent task");
    
    let result1 = process_user(123, "Alice");
    info!(result = %result1, "First user processed");
    
    let result2 = process_user(456, "Bob");  
    info!(result = %result2, "Second user processed");
    
    info!("Parent task completed");
}

fn main() {
    custom_tracing_logger::init();
    
    info!("=== #[instrument] Example ===");
    
    // Call instrumented functions
    parent_task();
    
    info!("=== Example completed ===");
    
    println!("\n💡 What you see:");
    println!("- 'enter' logs when functions start (with parameters)");
    println!("- 'exit' logs when functions end");
    println!("- All logs include span context with function parameters");
    println!("- Nested function calls show hierarchy");
}