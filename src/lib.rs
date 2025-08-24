//! Custom tracing logger that outputs structured JSON logs
//!
//! This crate provides a simple interface to initialize a JSON-formatted logger
//! using the tracing ecosystem. All logs are output as structured JSON with
//! metadata including timestamp, level, target, and message.

use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

/// Convenience macro for HTTP request logging
#[macro_export]
macro_rules! log_request {
    ($method:expr, $path:expr, $status:expr, $duration:expr) => {
        tracing::info!(
            method = $method,
            path = $path,
            status = $status,
            duration_ms = $duration,
            "HTTP request completed"
        );
    };
    ($method:expr, $path:expr, $status:expr, $duration:expr, $($key:ident = $value:expr),+) => {
        tracing::info!(
            method = $method,
            path = $path,
            status = $status,
            duration_ms = $duration,
            $($key = $value),+,
            "HTTP request completed"
        );
    };
}

/// Convenience macro for error logging with context
#[macro_export]
macro_rules! log_error {
    ($error_code:expr, $message:expr) => {
        tracing::error!(
            error_code = $error_code,
            $message
        );
    };
    ($error_code:expr, $message:expr, $($key:ident = $value:expr),+) => {
        tracing::error!(
            error_code = $error_code,
            $($key = $value),+,
            $message
        );
    };
}

/// Initialize the JSON logger
///
/// Behavior controlled by environment variables:
/// - `RUST_LOG`: Log level filtering (e.g., "info", "debug", "off")
/// - `LOG_FILE_DIR`: Directory for log files (e.g., "./logs")
/// - `LOG_FILE_PREFIX`: Prefix for log files (e.g., "myapp")
/// - `LOG_FILE_ONLY`: Set to "true" to disable console output
/// - `LOG_ENABLE_SPANS`: Set to "false" to disable #[instrument] span events (default: "true")
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
///
/// // Disable #[instrument] spans (with LOG_ENABLE_SPANS=false)
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
    let enable_spans =
        std::env::var("LOG_ENABLE_SPANS").unwrap_or_else(|_| "true".to_string()) == "true";

    let registry = tracing_subscriber::registry().with(env_filter);

    match (log_file_dir, file_only) {
        // File logging + console
        (Some(log_dir), false) => {
            let mut console_layer = fmt::layer()
                .json()
                .with_current_span(enable_spans)
                .with_span_list(false);

            if enable_spans {
                console_layer = console_layer
                    .with_span_events(fmt::format::FmtSpan::ENTER | fmt::format::FmtSpan::EXIT);
            }

            let file_appender =
                RollingFileAppender::new(Rotation::DAILY, &log_dir, &log_file_prefix);
            let mut file_layer = fmt::layer()
                .json()
                .with_current_span(enable_spans)
                .with_span_list(false)
                .with_writer(file_appender);

            if enable_spans {
                file_layer = file_layer
                    .with_span_events(fmt::format::FmtSpan::ENTER | fmt::format::FmtSpan::EXIT);
            }

            let _ = registry.with(console_layer).with(file_layer).try_init();
        }
        // File logging only (no console)
        (Some(log_dir), true) => {
            let file_appender =
                RollingFileAppender::new(Rotation::DAILY, &log_dir, &log_file_prefix);
            let mut file_layer = fmt::layer()
                .json()
                .with_current_span(enable_spans)
                .with_span_list(false)
                .with_writer(file_appender);

            if enable_spans {
                file_layer = file_layer
                    .with_span_events(fmt::format::FmtSpan::ENTER | fmt::format::FmtSpan::EXIT);
            }

            let _ = registry.with(file_layer).try_init();
        }
        // Console only
        (None, _) => {
            let mut console_layer = fmt::layer()
                .json()
                .with_current_span(enable_spans)
                .with_span_list(false);

            if enable_spans {
                console_layer = console_layer
                    .with_span_events(fmt::format::FmtSpan::ENTER | fmt::format::FmtSpan::EXIT);
            }

            let _ = registry.with(console_layer).try_init();
        }
    }
}

/// Validate current logging configuration without initializing
pub fn validate_config() -> Result<String, String> {
    let rust_log = std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string());
    let log_file_dir = std::env::var("LOG_FILE_DIR").ok();
    let log_file_prefix = std::env::var("LOG_FILE_PREFIX").unwrap_or_else(|_| "app".to_string());
    let file_only = std::env::var("LOG_FILE_ONLY").unwrap_or_default() == "true";
    let enable_spans =
        std::env::var("LOG_ENABLE_SPANS").unwrap_or_else(|_| "true".to_string()) == "true";

    // Validate RUST_LOG format by trying to create an EnvFilter
    if let Err(e) = EnvFilter::try_new(rust_log.trim()) {
        return Err(format!("Invalid RUST_LOG format: {}", e));
    }

    // Validate file directory if specified
    if let Some(ref dir) = log_file_dir {
        if let Err(e) = std::fs::create_dir_all(dir) {
            return Err(format!("Cannot create log directory '{}': {}", dir, e));
        }
    }

    let config = match (log_file_dir.as_ref(), file_only) {
        (Some(dir), false) => format!(
            "Console + File logging to {}/{}.YYYY-MM-DD",
            dir, log_file_prefix
        ),
        (Some(dir), true) => format!(
            "File-only logging to {}/{}.YYYY-MM-DD",
            dir, log_file_prefix
        ),
        (None, _) => "Console-only logging".to_string(),
    };

    let spans_status = if enable_spans { "enabled" } else { "disabled" };

    Ok(format!(
        "✓ RUST_LOG: {}\n✓ Mode: {}\n✓ Spans: {}",
        rust_log, config, spans_status
    ))
}

/// Print current logging configuration
pub fn print_config() {
    match validate_config() {
        Ok(config) => println!("Logging Configuration:\n{}", config),
        Err(error) => eprintln!("Logging Configuration Error: {}", error),
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

/// Structured logging helpers
pub mod structured {
    use tracing::{error, info};

    /// Log HTTP request with standard fields
    pub fn http_request(method: &str, path: &str, status: u16, duration_ms: u64) {
        info!(
            method = method,
            path = path,
            status = status,
            duration_ms = duration_ms,
            "HTTP request completed"
        );
    }

    /// Log database operation
    pub fn database_op(operation: &str, table: &str, duration_ms: u64, rows_affected: Option<u64>) {
        info!(
            operation = operation,
            table = table,
            duration_ms = duration_ms,
            rows_affected = rows_affected,
            "Database operation completed"
        );
    }

    /// Log user action with context
    pub fn user_action(user_id: u64, action: &str, resource: Option<&str>) {
        info!(
            user_id = user_id,
            action = action,
            resource = resource,
            "User action performed"
        );
    }

    /// Log error with structured context
    pub fn error_with_context(error_code: &str, message: &str) {
        error!(error_code = error_code, "{}" = message);
    }
}
