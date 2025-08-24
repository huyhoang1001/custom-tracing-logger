//! Example: Web server logging with convenience macros
//! Run with: cargo run --example web_server

use custom_tracing_logger::{log_error, log_request};
use tracing::{info, instrument, warn};

#[instrument]
fn authenticate_user(token: &str) -> Result<u64, &'static str> {
    info!("Checking authentication token");

    if token == "valid_token" {
        info!("Authentication successful");
        Ok(123) // user_id
    } else {
        log_error!(
            "AUTH_FAILED",
            "Invalid authentication token",
            token_prefix = &token[..token.len().min(4)]
        );
        Err("Invalid token")
    }
}

#[instrument]
fn handle_request(method: &str, path: &str) -> (u16, String) {
    match path {
        "/api/users" => match authenticate_user("valid_token") {
            Ok(user_id) => {
                log_request!(method, path, 200, 45, user_id = user_id);
                (200, "User data".to_string())
            }
            Err(_) => {
                log_request!(method, path, 401, 12);
                (401, "Unauthorized".to_string())
            }
        },
        "/health" => {
            log_request!(method, path, 200, 5);
            (200, "OK".to_string())
        }
        _ => {
            warn!(method = method, path = path, "Route not found");
            log_request!(method, path, 404, 8);
            (404, "Not Found".to_string())
        }
    }
}

fn main() {
    custom_tracing_logger::init();

    info!("Web server starting");

    // Simulate requests
    let requests = vec![
        ("GET", "/api/users"),
        ("GET", "/health"),
        ("POST", "/unknown"),
    ];

    for (method, path) in requests {
        let (status, _response) = handle_request(method, path);
        info!(
            method = method,
            path = path,
            status = status,
            "Request completed"
        );
    }

    info!("Web server stopped");
}
