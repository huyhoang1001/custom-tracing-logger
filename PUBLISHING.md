# Publishing Guide

## Prerequisites

1. **Create a crates.io account**: Go to https://crates.io and sign up with GitHub
2. **Get API token**: Go to https://crates.io/me and create a new API token
3. **Login to cargo**: Run `cargo login` and paste your API token

## Before Publishing

### 1. Update Cargo.toml
Replace placeholder values in `Cargo.toml`:
```toml
authors = ["Your Name <your.email@example.com>"]
repository = "https://github.com/yourusername/custom-tracing-logger"
homepage = "https://github.com/yourusername/custom-tracing-logger"
```

### 2. Update LICENSE
Replace "Your Name" in `LICENSE` file with your actual name.

### 3. Exclude Unnecessary Files
Update `exclude` in `Cargo.toml` to keep package size small:
```toml
exclude = [
    "target/",           # Build artifacts
    "logs/",             # Log files
    "logs_*/",           # Log directories
    "BLOG.md",           # Blog content
    "PUBLISHING.md",     # Publishing guide
    "examples/debug_*.rs", # Debug examples
    "examples/test_*.rs",  # Test examples
    ".env",              # Environment files
    "*.log",             # Log files
    ".git/",             # Git directory
    "*.tmp",             # Temporary files
]
```

### 4. Final Checks
```bash
# Run tests
cargo test

# Check package contents
cargo package --list

# Build package
cargo package
```

## Publishing Steps

### 1. First Time Publishing
```bash
cargo publish
```

### 2. Future Updates
1. Update version in `Cargo.toml` (e.g., `0.1.1`, `0.2.0`)
2. Update `CHANGELOG.md` if you have one
3. Run tests: `cargo test`
4. Publish: `cargo publish`

## Versioning Guidelines

- **Patch** (0.1.0 → 0.1.1): Bug fixes, no breaking changes
- **Minor** (0.1.0 → 0.2.0): New features, no breaking changes  
- **Major** (0.1.0 → 1.0.0): Breaking changes

## Post-Publishing

1. **Documentation**: Will be automatically generated at https://docs.rs/custom-tracing-logger
2. **GitHub**: Create a repository and push your code
3. **README**: Update installation instructions with the published crate name

## Troubleshooting

- **Name taken**: Choose a different name in `Cargo.toml`
- **Missing metadata**: Add required fields to `[package]` section
- **Build fails**: Fix any compilation errors before publishing
- **Version exists**: Increment version number

## Example Usage After Publishing

```toml
[dependencies]
custom-tracing-logger = "0.1.0"
tracing = "0.1"
```

```rust
fn main() {
    custom_tracing_logger::init();
    tracing::info!("Hello, world!");
}
```