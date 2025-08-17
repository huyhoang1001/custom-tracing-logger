# Custom Tracing Logger

A minimal Rust crate for structured JSON logging using the `tracing` ecosystem.

## Features

- **JSON Output**: All logs formatted as structured JSON
- **Simple API**: One-line initialization
- **Selective Monitoring**: Control logging per module
- **Environment Variable Support**: Respects `RUST_LOG`
- **Extensible**: Built on `tracing-subscriber` layers

## Quick Start

Add to `Cargo.toml`:
```toml
[dependencies]
custom-tracing-logger = "0.1.0"
tracing = "0.1"
```

Basic usage:
```rust
use tracing::info;

fn main() {
    custom_tracing_logger::init();
    info!(user_id = 123, "User logged in");
}
```

Output:
```json
{"timestamp":"2025-08-17T08:47:20.336668Z","level":"INFO","fields":{"message":"User logged in","user_id":123},"target":"my_app"}
```

## API

### `init()`
One function, behavior controlled by environment variables:
```rust
custom_tracing_logger::init();
```

**Environment Variables:**
- `RUST_LOG`: Log level filtering (e.g., "info", "debug", "off")
- `LOG_FILE_DIR`: Directory for log files (e.g., "./logs")
- `LOG_FILE_PREFIX`: Prefix for log files (default: "app")
- `LOG_FILE_ONLY`: Set to "true" to disable console output

## Filtering Examples

### Console Only (Default)
```bash
RUST_LOG=info cargo run
```

### Console + File Logging
```bash
RUST_LOG=info LOG_FILE_DIR=./logs LOG_FILE_PREFIX=myapp cargo run
```

### File Only (Silent Console)
```bash
RUST_LOG=info LOG_FILE_DIR=./logs LOG_FILE_ONLY=true cargo run
```

### Module Filtering
```powershell
# Specific modules
$env:RUST_LOG='myapp::auth_service=info,myapp::payment_service=warn'; cargo run

# Turn off noisy modules
$env:RUST_LOG='debug,tokio=info,hyper=warn'; cargo run
```

## Examples

Run examples:
```powershell
cargo run --example basic
$env:RUST_LOG='off,selective::auth_service=info,selective::payment_service=warn'; cargo run --example selective
$env:RUST_LOG='debug'; cargo run --example filtering
cargo run --example with_dotenv  # Uses .env file
$env:RUST_LOG='info'; $env:LOG_FILE_DIR='./logs'; $env:LOG_FILE_PREFIX='myapp'; cargo run --example file_logging
$env:RUST_LOG='info'; $env:LOG_FILE_DIR='./logs'; $env:LOG_FILE_ONLY='true'; cargo run --example file_only
```

### Using .env File
Create `.env` in your project root:
```bash
# .env
RUST_LOG=info
LOG_FILE_DIR=./logs
LOG_FILE_PREFIX=myapp
# LOG_FILE_ONLY=true  # Uncomment for file-only logging
```

Then load it before initializing:
```rust
dotenv::dotenv().ok();
custom_tracing_logger::init();
```

## JSON Output Format

```json
{
  "timestamp": "2025-08-17T08:47:20.336668Z",
  "level": "INFO",
  "fields": {
    "message": "HTTP request completed",
    "request_id": "req-abc123",
    "method": "GET",
    "status": 200
  },
  "target": "my_web_server"
}
```

## License

MIT