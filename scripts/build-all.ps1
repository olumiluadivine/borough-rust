# PowerShell build script for Windows

Write-Host "Building all services..." -ForegroundColor Green

# Build shared library first
Write-Host "Building shared library..." -ForegroundColor Yellow
Set-Location shared
cargo build --release
Set-Location ..

# Build all services
$services = @("auth-service", "user-service", "property-service", "booking-service", "transaction-service", "notification-service", "feedback-service", "search-service", "external-comm-service")

foreach ($service in $services) {
    Write-Host "Building $service..." -ForegroundColor Yellow
    Set-Location "services\$service"
    cargo build --release
    Set-Location "..\..\"
}

Write-Host "All services built successfully!" -ForegroundColor Green
