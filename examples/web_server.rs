//! Example: Web server logging patterns
//! Run with: $env:RUST_LOG='info'; cargo run --example web_server

use custom_tracing_logger::{self, log_request, structured};
use tracing::{info, warn, error, instrument};
use std::time::Instant;

#[derive(Debug)]
struct User {
    id: u64,
    name: String,
}

#[instrument]
async fn authenticate_user(token: &str) -> Result<User, &'static str> {
    let start = Instant::now();
    
    // Simulate auth logic
    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
    
    if token == "valid_token" {
        let user = User { id: 123, name: "Alice".to_string() };
        
        structured::user_action(user.id, "login", Some("web"));
        structured::database_op("SELECT", "users", start.elapsed().as_millis() as u64, Some(1));
        
        Ok(user)
    } else {
        error!(
            error_code = "AUTH_FAILED",
            token_prefix = &token[..token.len().min(4)],
            "Authentication failed"
        );
        Err("Invalid token")
    }
}

#[instrument]
async fn handle_request(method: &str, path: &str) -> (u16, String) {
    let start = Instant::now();
    
    match path {
        "/api/users" => {
            match authenticate_user("valid_token").await {
                Ok(user) => {
                    log_request!(method, path, 200, start.elapsed().as_millis() as u64, user_id = user.id);
                    (200, format!("Hello, {}!", user.name))
                }
                Err(_) => {
                    log_request!(method, path, 401, start.elapsed().as_millis() as u64);
                    (401, "Unauthorized".to_string())
                }
            }
        }
        "/health" => {
            log_request!(method, path, 200, start.elapsed().as_millis() as u64);
            (200, "OK".to_string())
        }
        _ => {
            warn!(
                method = method,
                path = path,
                "Route not found"
            );
            log_request!(method, path, 404, start.elapsed().as_millis() as u64);
            (404, "Not Found".to_string())
        }
    }
}

#[tokio::main]
async fn main() {
    custom_tracing_logger::init();
    
    info!("Web server starting up");
    custom_tracing_logger::print_config();
    
    // Simulate some requests
    let requests = vec![
        ("GET", "/api/users"),
        ("GET", "/health"),
        ("POST", "/api/unknown"),
        ("GET", "/api/users"),
    ];
    
    for (method, path) in requests {
        let (status, response) = handle_request(method, path).await;
        info!(
            method = method,
            path = path,
            status = status,
            response_length = response.len(),
            "Request processed"
        );
    }
    
    info!("Web server shutting down");
}