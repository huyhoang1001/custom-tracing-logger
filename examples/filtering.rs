//! Examples of different filtering approaches
//!
//! Try these commands:
//! cargo run --example filtering
//! $env:RUST_LOG='debug'; cargo run --example filtering  
//! $env:RUST_LOG='filtering::auth_service=info,filtering::payment_service=warn'; cargo run --example filtering

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
    // This respects RUST_LOG environment variable
    custom_tracing_logger::init();

    info!("=== Application Started ===");

    auth_service::authenticate();
    payment_service::charge_card();

    debug!("This debug message from main");
    info!("=== Application Finished ===");
}
