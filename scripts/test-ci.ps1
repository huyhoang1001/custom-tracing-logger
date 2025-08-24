# Local CI testing script
# Run this to simulate GitHub Actions locally

Write-Host "Running Local CI Tests" -ForegroundColor Green

# Test 1: Run tests
Write-Host "`nRunning Tests..." -ForegroundColor Yellow
cargo test --verbose
if ($LASTEXITCODE -ne 0) {
    Write-Host "Tests failed!" -ForegroundColor Red
    exit 1
}

# Test 2: Check formatting
Write-Host "`nChecking Code Formatting..." -ForegroundColor Yellow
cargo fmt --all -- --check
if ($LASTEXITCODE -ne 0) {
    Write-Host "Code formatting issues found!" -ForegroundColor Red
    Write-Host "Run: cargo fmt --all" -ForegroundColor Cyan
    exit 1
}

# Test 3: Run clippy
Write-Host "`nRunning Clippy..." -ForegroundColor Yellow
cargo clippy --all-targets --all-features -- -D warnings
if ($LASTEXITCODE -ne 0) {
    Write-Host "Clippy found issues!" -ForegroundColor Red
    exit 1
}

# Test 4: Build examples
Write-Host "`nBuilding Examples..." -ForegroundColor Yellow
cargo build --examples
if ($LASTEXITCODE -ne 0) {
    Write-Host "Examples failed to build!" -ForegroundColor Red
    exit 1
}

# Test 5: Run examples
Write-Host "`nTesting Examples..." -ForegroundColor Yellow
cargo run --example simple_usage
cargo run --example configuration

# Test 6: Security audit (if cargo-audit is installed)
Write-Host "`nRunning Security Audit..." -ForegroundColor Yellow
if (Get-Command cargo-audit -ErrorAction SilentlyContinue) {
    cargo audit
    if ($LASTEXITCODE -ne 0) {
        Write-Host "Security vulnerabilities found!" -ForegroundColor Yellow
    }
} else {
    Write-Host "cargo-audit not installed. Run: cargo install cargo-audit" -ForegroundColor Cyan
}

Write-Host "`nAll CI checks passed!" -ForegroundColor Green
Write-Host "Ready to push to GitHub!" -ForegroundColor Green