//! Example showing selective module monitoring using RUST_LOG
//! 
//! Run with:
//! $env:RUST_LOG='off,selective::auth_service=info,selective::payment_service=warn'; cargo run --example selective

use custom_tracing_logger;
use tracing::info;

// Simulate different modules
mod auth_service {
    use tracing::info;
    pub fn login() {
        info!("User login attempt");
    }
}

mod payment_service {
    use tracing::warn;
    pub fn process_payment() {
        warn!("Payment processing started");
    }
}

mod noisy_module {
    use tracing::debug;
    pub fn spam_logs() {
        for i in 0..5 {
            debug!("Noisy debug message {}", i);
        }
    }
}

#[tokio::main]
async fn main() {
    // Simple init - all filtering controlled by RUST_LOG
    custom_tracing_logger::init();

    info!("App started");
    
    auth_service::login();
    payment_service::process_payment();
    noisy_module::spam_logs();
    
    info!("App finished");
}