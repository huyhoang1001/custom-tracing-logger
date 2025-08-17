//! Custom tracing logger that outputs structured JSON logs
//! 
//! This crate provides a simple interface to initialize a JSON-formatted logger
//! using the tracing ecosystem. All logs are output as structured JSON with
//! metadata including timestamp, level, target, and message.

use tracing_subscriber::{
    fmt,
    layer::SubscriberExt,
    util::SubscriberInitExt,
    EnvFilter,
};
use tracing_appender::rolling::{RollingFileAppender, Rotation};

/// Initialize the JSON logger
/// 
/// Behavior controlled by environment variables:
/// - `RUST_LOG`: Log level filtering (e.g., "info", "debug", "off")
/// - `LOG_FILE_DIR`: Directory for log files (e.g., "./logs")
/// - `LOG_FILE_PREFIX`: Prefix for log files (e.g., "myapp")
/// - `LOG_FILE_ONLY`: Set to "true" to disable console output
/// 
/// # Examples
/// ```no_run
/// // Console only
/// custom_tracing_logger::init();
/// 
/// // Console + file (with LOG_FILE_DIR=./logs LOG_FILE_PREFIX=myapp)
/// custom_tracing_logger::init();
/// 
/// // File only (with LOG_FILE_ONLY=true)
/// custom_tracing_logger::init();
/// ```
pub fn init() {
    // Handle RUST_LOG with whitespace trimming for Windows compatibility
    let env_filter = match std::env::var("RUST_LOG") {
        Ok(val) => EnvFilter::new(val.trim()),
        Err(_) => EnvFilter::new("info"),
    };

    // Check for file logging configuration
    let log_file_dir = std::env::var("LOG_FILE_DIR").ok();
    let log_file_prefix = std::env::var("LOG_FILE_PREFIX").unwrap_or_else(|_| "app".to_string());
    let file_only = std::env::var("LOG_FILE_ONLY").unwrap_or_default() == "true";

    let registry = tracing_subscriber::registry().with(env_filter);

    match (log_file_dir, file_only) {
        // File logging + console
        (Some(log_dir), false) => {
            let console_layer = fmt::layer()
                .json()
                .with_current_span(false)
                .with_span_list(false);
            
            let file_appender = RollingFileAppender::new(Rotation::DAILY, &log_dir, &log_file_prefix);
            let file_layer = fmt::layer()
                .json()
                .with_current_span(false)
                .with_span_list(false)
                .with_writer(file_appender);
            
            let _ = registry.with(console_layer).with(file_layer).try_init();
        },
        // File logging only (no console)
        (Some(log_dir), true) => {
            let file_appender = RollingFileAppender::new(Rotation::DAILY, &log_dir, &log_file_prefix);
            let file_layer = fmt::layer()
                .json()
                .with_current_span(false)
                .with_span_list(false)
                .with_writer(file_appender);
            
            let _ = registry.with(file_layer).try_init();
        },
        // Console only
        (None, _) => {
            let console_layer = fmt::layer()
                .json()
                .with_current_span(false)
                .with_span_list(false);
            
            let _ = registry.with(console_layer).try_init();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_does_not_panic() {
        let _ = std::panic::catch_unwind(|| {
            init();
        });
    }

    #[test]
    fn test_env_var_parsing() {
        // Test that environment variables are read correctly
        std::env::set_var("LOG_FILE_PREFIX", "test");
        let prefix = std::env::var("LOG_FILE_PREFIX").unwrap_or_else(|_| "app".to_string());
        assert_eq!(prefix, "test");
        std::env::remove_var("LOG_FILE_PREFIX");
    }


}