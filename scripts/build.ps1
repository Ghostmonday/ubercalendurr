#!/usr/bin/env pwsh

$ErrorActionPreference = "Stop"
$ProgressPreference = "SilentlyContinue"

$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$repoRoot = Split-Path -Parent $scriptDir

Write-Host "=== UberCalendurr Build Script ===" -ForegroundColor Cyan
Write-Host ""

# Check Rust installation
if (-not (Get-Command rustc -ErrorAction SilentlyContinue)) {
    Write-Error "Rust is not installed. Please install from https://rustup.rs/"
    exit 1
}

Write-Host "Rust version: $(rustc --version)"
Write-Host "Cargo version: $(cargo --version)"
Write-Host ""

# Change to repo root
Push-Location $repoRoot

try {
    # Fetch dependencies
    Write-Host "Fetching dependencies..." -ForegroundColor Gray
    cargo fetch
    
    # Build workspace
    Write-Host "Building workspace..." -ForegroundColor Gray
    cargo build --workspace
    
    if ($LASTEXITCODE -ne 0) {
        throw "Build failed"
    }
    
    Write-Host ""
    Write-Host "Build completed successfully!" -ForegroundColor Green
    
    # Build frontend if needed
    $frontendDir = Join-Path $repoRoot "binaries/calendar-gui/frontend"
    if (Test-Path $frontendDir) {
        Write-Host ""
        Write-Host "Building frontend..." -ForegroundColor Gray
        Push-Location $frontendDir
        try {
            npm install
            npm run build
        }
        finally {
            Pop-Location
        }
    }
    
    Write-Host ""
    Write-Host "All builds completed!" -ForegroundColor Green
}
catch {
    Write-Error "Build failed: $_"
    exit 1
}
finally {
    Pop-Location
}
