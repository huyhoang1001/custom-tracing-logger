//! Example using .env file for configuration
//!
//! Create a .env file in the project root with:
//! RUST_LOG=off,auth_service=info,payment_service=warn
//!
//! Run with: cargo run --example with_dotenv

use tracing::{debug, info};

mod auth_service {
    use tracing::{debug, info};
    pub fn authenticate() {
        debug!("Checking credentials");
        info!("User authenticated");
    }
}

mod payment_service {
    use tracing::{error, warn};
    pub fn charge_card() {
        warn!("Processing payment");
        error!("Payment failed - insufficient funds");
    }
}

#[tokio::main]
async fn main() {
    // Load .env file before initializing logger
    dotenv::dotenv().ok();

    // Initialize logger (will use RUST_LOG from .env)
    custom_tracing_logger::init();

    info!("=== Application Started ===");

    auth_service::authenticate();
    payment_service::charge_card();

    debug!("This debug message from main");
    info!("=== Application Finished ===");
}
