# Multi-stage build for auth-service
FROM rust:1.75 AS builder

WORKDIR /app

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY shared/ ./shared/
COPY services/auth-service/ ./services/auth-service/

# Build the application
RUN cargo build --release --bin auth-service

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Create app user
RUN useradd -m -u 1001 app

WORKDIR /app

# Copy the binary
COPY --from=builder /app/target/release/auth-service ./auth-service

# Change ownership
RUN chown -R app:app /app
USER app

EXPOSE 8001 9001

CMD ["./auth-service"]
