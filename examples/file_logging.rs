//! Example showing file logging using environment variables
//!
//! Run with: $env:RUST_LOG='info'; $env:LOG_FILE_DIR='./logs'; $env:LOG_FILE_PREFIX='myapp'; cargo run --example file_logging
//! Check ./logs/ directory for log files

use tracing::{error, info, warn};

#[tokio::main]
async fn main() {
    // Single init - behavior controlled by environment variables
    custom_tracing_logger::init();

    info!("Application started - this goes to both console and file");

    warn!(
        user_id = 123,
        action = "login_attempt",
        "User login attempt"
    );

    error!(
        error_code = "DB_TIMEOUT",
        retry_count = 3,
        "Database connection timeout"
    );

    info!("Check ./logs/myapp.YYYY-MM-DD for the log file");
}
