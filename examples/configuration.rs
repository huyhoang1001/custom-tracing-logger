//! Example: Different configuration options
//! Run with different environment variables to see the effects

use tracing::{debug, info, instrument, warn};

#[instrument]
fn sample_operation(operation_id: u64) {
    info!(operation_id = operation_id, "Starting operation");

    debug!("This debug message shows with RUST_LOG=debug");

    // Simulate some work
    std::thread::sleep(std::time::Duration::from_millis(10));

    warn!(
        operation_id = operation_id,
        "Operation completed with warnings"
    );
}

fn main() {
    custom_tracing_logger::init();

    // Print current configuration
    custom_tracing_logger::print_config();

    info!("=== Configuration Example ===");

    // Run some operations
    sample_operation(1001);
    sample_operation(1002);

    info!("=== Example completed ===");

    println!("\n💡 Try these configurations:");
    println!("Default:                    cargo run --example configuration");
    println!(
        "Debug level:                $env:RUST_LOG='debug'; cargo run --example configuration"
    );
    println!(
        "File logging:               $env:LOG_FILE_DIR='./logs'; cargo run --example configuration"
    );
    println!("File only:                  $env:LOG_FILE_DIR='./logs'; $env:LOG_FILE_ONLY='true'; cargo run --example configuration");
    println!("Disable spans:              $env:LOG_ENABLE_SPANS='false'; cargo run --example configuration");
}
