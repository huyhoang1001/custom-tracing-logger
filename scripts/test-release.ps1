# Local release validation script
# Run this before creating a release tag

param(
    [Parameter(Mandatory=$true)]
    [string]$Version
)

Write-Host "Testing Release v$Version" -ForegroundColor Green

# Test 1: Verify Cargo.toml version matches
Write-Host "`nChecking Version Consistency..." -ForegroundColor Yellow
$cargoVersion = (cargo metadata --no-deps --format-version 1 | ConvertFrom-Json).packages[0].version
if ($cargoVersion -ne $Version) {
    Write-Host "❌ Version mismatch!" -ForegroundColor Red
    Write-Host "Cargo.toml version: $cargoVersion" -ForegroundColor Red
    Write-Host "Requested version: $Version" -ForegroundColor Red
    Write-Host "Update Cargo.toml version to match!" -ForegroundColor Cyan
    exit 1
}

# Test 2: Run full CI
Write-Host "`nRunning Full CI..." -ForegroundColor Yellow
& "$PSScriptRoot\test-ci.ps1"
if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ CI tests failed!" -ForegroundColor Red
    exit 1
}

# Test 3: Check if tag already exists
Write-Host "`nChecking Git Tags..." -ForegroundColor Yellow
$existingTag = git tag -l "v$Version"
if ($existingTag) {
    Write-Host "❌ Tag v$Version already exists!" -ForegroundColor Red
    exit 1
}

# Test 4: Check changelog
Write-Host "`nChecking Changelog..." -ForegroundColor Yellow
if (!(Test-Path "CHANGELOG.md")) {
    Write-Host "⚠️ CHANGELOG.md not found!" -ForegroundColor Yellow
} else {
    $changelog = Get-Content "CHANGELOG.md" -Raw
    if ($changelog -notmatch "\[$Version\]") {
        Write-Host "⚠️ Version $Version not found in CHANGELOG.md" -ForegroundColor Yellow
    }
}

Write-Host "`nRelease validation passed!" -ForegroundColor Green
Write-Host "Ready to create release:" -ForegroundColor Green
Write-Host "  git tag v$Version" -ForegroundColor Cyan
Write-Host "  git push origin v$Version" -ForegroundColor Cyan