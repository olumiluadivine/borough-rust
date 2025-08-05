#!/bin/bash

# Build script for all services

echo "Building all services..."

# Build shared library first
echo "Building shared library..."
cd shared && cargo build --release
cd ..

# Build all services
services=("auth-service" "user-service" "property-service" "booking-service" "transaction-service" "notification-service" "feedback-service" "search-service" "external-comm-service")

for service in "${services[@]}"; do
    echo "Building $service..."
    cd "services/$service"
    cargo build --release
    cd "../.."
done

echo "All services built successfully!"
