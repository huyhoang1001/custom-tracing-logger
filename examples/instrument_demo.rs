//! Demonstration of #[instrument] for debugging
//! Run with: $env:RUST_LOG='debug'; cargo run --example instrument_demo

use custom_tracing_logger;
use tracing::{info, debug, warn, instrument};

// Basic instrumentation - logs function entry/exit
#[instrument]
fn calculate_sum(a: i32, b: i32) -> i32 {
    debug!("Performing addition");
    let result = a + b;
    info!(result = result, "Calculation completed");
    result
}

// Instrumentation with custom level
#[instrument(level = "info")]
async fn fetch_user_data(user_id: u64) -> Result<String, &'static str> {
    info!("Starting user data fetch");
    
    // Simulate some async work
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    
    if user_id == 0 {
        warn!("Invalid user ID provided");
        return Err("Invalid user ID");
    }
    
    let user_data = format!("User data for ID: {}", user_id);
    info!(data_length = user_data.len(), "User data fetched successfully");
    Ok(user_data)
}

// Skip certain parameters from logging
#[instrument(skip(password))]
fn authenticate_user(username: &str, password: &str) -> bool {
    info!("Authenticating user");
    
    // Don't log the password for security
    if username == "admin" && password == "secret123" {
        info!("Authentication successful");
        true
    } else {
        warn!("Authentication failed");
        false
    }
}

// Custom span name and fields
#[instrument(name = "db_operation", fields(table = "users", operation = "SELECT"))]
async fn query_database(query: &str) -> Result<Vec<String>, &'static str> {
    info!(query = query, "Executing database query");
    
    // Simulate database work
    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
    
    if query.contains("DROP") {
        warn!("Dangerous query detected");
        return Err("Dangerous operation not allowed");
    }
    
    let results = vec!["row1".to_string(), "row2".to_string()];
    info!(row_count = results.len(), "Query completed successfully");
    Ok(results)
}

// Nested function calls show call hierarchy
#[instrument]
async fn process_user_request(user_id: u64, username: &str, password: &str) -> Result<String, String> {
    info!("Processing user request");
    
    // This creates a nested span structure
    if !authenticate_user(username, password) {
        return Err("Authentication failed".to_string());
    }
    
    let user_data = fetch_user_data(user_id).await
        .map_err(|e| format!("Failed to fetch user data: {}", e))?;
    
    let query = format!("SELECT * FROM users WHERE id = {}", user_id);
    let _db_results = query_database(&query).await
        .map_err(|e| format!("Database error: {}", e))?;
    
    let sum = calculate_sum(user_id as i32, 42);
    
    info!(final_sum = sum, "Request processing completed");
    Ok(format!("Processed: {} (sum: {})", user_data, sum))
}

#[tokio::main]
async fn main() {
    custom_tracing_logger::init();
    
    info!("=== Instrument Demo Starting ===");
    
    // Test successful case
    match process_user_request(123, "admin", "secret123").await {
        Ok(result) => info!(result = result, "Request succeeded"),
        Err(e) => warn!(error = e, "Request failed"),
    }
    
    info!("---");
    
    // Test authentication failure
    match process_user_request(456, "user", "wrong_password").await {
        Ok(result) => info!(result = result, "Request succeeded"),
        Err(e) => warn!(error = e, "Request failed"),
    }
    
    info!("---");
    
    // Test invalid user ID
    match process_user_request(0, "admin", "secret123").await {
        Ok(result) => info!(result = result, "Request succeeded"),
        Err(e) => warn!(error = e, "Request failed"),
    }
    
    info!("=== Instrument Demo Completed ===");
}