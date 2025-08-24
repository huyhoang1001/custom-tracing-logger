# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.1] - 2025-01-15

### Added
- `LOG_ENABLE_SPANS` environment variable to control #[instrument] span events
- Convenience macros: `log_request!` and `log_error!`
- Structured logging helpers in `structured` module
- Configuration validation with `validate_config()` and `print_config()`
- Comprehensive examples for different use cases

### Changed
- Enabled span events by default for better #[instrument] support
- Improved JSON output with span context when spans are enabled

### Fixed
- Windows compatibility for environment variable parsing
- Proper span event configuration in JSON formatter

## [0.1.0] - 2025-01-15

### Added
- Initial release
- Basic JSON logging with `init()` function
- Environment variable configuration (`RUST_LOG`, `LOG_FILE_DIR`, `LOG_FILE_PREFIX`, `LOG_FILE_ONLY`)
- Daily rotating file logs
- Console and file logging support