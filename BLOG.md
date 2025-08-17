# Why I Built a One-Line JSON Logger for Rust

*TL;DR: Tired of configuring tracing-subscriber for every project? This crate gives you production-ready JSON logging with just `custom_tracing_logger::init()`.*

## The Problem

You know the drill. You're building a Rust service, everything works great locally with `println!` debugging, but then you need to deploy it. Suddenly you're wrestling with:

```rust
// The usual tracing-subscriber dance
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

fn setup_logging() {
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));
    
    let json_layer = fmt::layer()
        .json()
        .with_current_span(false)
        .with_span_list(false);
    
    tracing_subscriber::registry()
        .with(env_filter)
        .with(json_layer)
        .init();
}
```

Every. Single. Project.

## Understanding the Tracing Ecosystem

Before we dive into the solution, let's understand what we're working with:

**`tracing`** - The instrumentation framework. This is what you import and use in your code:
```rust
use tracing::{info, warn, error};
info!(user_id = 123, "User logged in");
```
It's lightweight, fast, and provides structured logging with fields. Think of it as the "what to log" part.

**`tracing-subscriber`** - The heavy lifter that handles:
- **Formatting**: JSON, plain text, or custom formats
- **Filtering**: Which logs to show (by level, module, etc.)
- **Output**: Where logs go (console, files, network, etc.)

Think of it as the "how to log" part.

**The Problem**: Setting up `tracing-subscriber` correctly requires understanding layers, registries, filters, and formatters. It's powerful but verbose.

**The Solution**: Hide all that complexity behind one function call.

## What I Wanted

```rust
fn main() {
    custom_tracing_logger::init();  // One function, all behaviors
    
    tracing::info!(user_id = 123, "User logged in");
}
```

That's it. One function. Behavior controlled by environment variables. No boilerplate. No copy-pasting subscriber setup. Just structured JSON logs ready for your ELK stack.

## The Output

Instead of this human-readable mess:
```
2024-01-15T10:30:45.123456Z  INFO my_app: User logged in user_id=123
```

You get this machine-readable beauty:
```json
{
  "timestamp": "2025-08-17T09:14:27.328229Z",
  "level": "INFO",
  "fields": {
    "message": "User logged in",
    "user_id": 123
  },
  "target": "my_app"
}
```

## Why JSON Logs Matter

**For Humans**: JSON logs look ugly in your terminal, but you're not the audience in production.

**For Machines**: Your monitoring stack loves structured data. Elasticsearch can index it. Grafana can graph it. Your alerting system can parse it without regex nightmares.

**For Teams**: Consistent log format across all services means your DevOps team doesn't hate you.

## The Best Part: RUST_LOG Still Works

Our crate is built on `tracing-subscriber`'s `EnvFilter`, so all the filtering you know and love still works:

```powershell
# Development: see everything
$env:RUST_LOG='debug'; cargo run

# Production: only errors
$env:RUST_LOG='error'; cargo run

# Selective monitoring: ignore noisy modules
$env:RUST_LOG='info,tokio=warn,hyper=error'; cargo run

# .env file support
echo "RUST_LOG=off,myapp::auth_service=info,myapp::payment_service=warn" > .env
```

**Why This Matters**: You're not learning a new filtering system. If you know `RUST_LOG`, you already know how to control our logger.

## Real-World Usage

```rust
use tracing::{info, warn, error};

#[tokio::main]
async fn main() {
    // One init, behavior controlled by environment
    custom_tracing_logger::init();
    
    // Structured logging with context
    info!(
        request_id = "req-abc123",
        method = "POST",
        path = "/api/users",
        duration_ms = 45,
        "Request completed"
    );
    
    // Error with details
    error!(
        error_code = "DB_CONNECTION_FAILED",
        retry_count = 3,
        database = "users_db",
        "Database connection failed"
    );
}
```

**Environment Variables Control Everything:**
```powershell
# Development: console only
$env:RUST_LOG='info'; cargo run

# Production: console + daily rotating files
$env:RUST_LOG='info'; $env:LOG_FILE_DIR='./logs'; $env:LOG_FILE_PREFIX='myservice'; cargo run

# Background service: file only
$env:RUST_LOG='info'; $env:LOG_FILE_DIR='/var/log'; $env:LOG_FILE_ONLY='true'; cargo run
```

Output ready for your log aggregator:
```json
{"timestamp":"2025-08-17T09:14:27.328229Z","level":"INFO","fields":{"message":"Request completed","request_id":"req-abc123","method":"POST","path":"/api/users","duration_ms":45},"target":"my_service"}
{"timestamp":"2025-08-17T09:14:27.328479Z","level":"ERROR","fields":{"message":"Database connection failed","error_code":"DB_CONNECTION_FAILED","retry_count":3,"database":"users_db"},"target":"my_service"}
```

## Environment Variable Magic

**One Function, Multiple Behaviors:**
- `RUST_LOG=info` → Console logging
- `LOG_FILE_DIR=./logs` → Add file logging  
- `LOG_FILE_ONLY=true` → File only (silent console)
- `LOG_FILE_PREFIX=myapp` → Custom file prefix

**Daily Rotation**: Files automatically rotate (`myapp.2024-01-15`)
**Same JSON Format**: Consistent structure everywhere
**Runtime Control**: Change behavior without recompiling

## When NOT to Use This

- **Local development**: Human-readable logs are better for debugging
- **Simple scripts**: If you're not shipping to production, default tracing is fine
- **Custom formatting needs**: This is opinionated - if you need different JSON structure, stick with tracing-subscriber

## Installation

```toml
[dependencies]
custom-tracing-logger = "0.1.0"
tracing = "0.1"
```

## One Function, All Use Cases

`custom_tracing_logger::init()` - Everything controlled by environment variables:

- **Development**: Just `$env:RUST_LOG='info'`
- **Production**: Add `$env:LOG_FILE_DIR='./logs'`  
- **Background Services**: Add `$env:LOG_FILE_ONLY='true'`

Same clean JSON format everywhere. Runtime configuration without code changes.

## The Philosophy

This crate doesn't reinvent logging. It's a thin wrapper around the excellent `tracing` ecosystem that eliminates the setup boilerplate for the 90% use case: **structured JSON logs in production**.

One function. One line. Production-ready logging.

Sometimes the best code is the code you don't have to write.

---

*Built with ❤️ for Rust developers who have better things to do than configure logging.*