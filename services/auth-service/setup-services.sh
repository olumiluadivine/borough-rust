#!/bin/bash

# setup-services.sh - Script to spin up required services for auth service
# This script starts PostgreSQL, Redis, and RabbitMQ containers and updates .env file

set -e

echo "🚀 Starting required services for auth service..."

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Function to check if docker is running
check_docker() {
    if ! docker info >/dev/null 2>&1; then
        echo -e "${RED}❌ Docker is not running. Please start Docker and try again.${NC}"
        exit 1
    fi
    echo -e "${GREEN}✅ Docker is running${NC}"
}

# Function to wait for service to be ready
wait_for_service() {
    local service_name=$1
    local port=$2
    local max_attempts=30
    local attempt=1
    
    echo -e "${YELLOW}⏳ Waiting for $service_name to be ready on port $port...${NC}"
    
    while [ $attempt -le $max_attempts ]; do
        if nc -z localhost $port 2>/dev/null; then
            echo -e "${GREEN}✅ $service_name is ready!${NC}"
            return 0
        fi
        echo "Attempt $attempt/$max_attempts - $service_name not ready yet..."
        sleep 2
        attempt=$((attempt + 1))
    done
    
    echo -e "${RED}❌ $service_name failed to start within expected time${NC}"
    return 1
}

# Function to start PostgreSQL
start_postgres() {
    echo -e "${YELLOW}🐘 Starting PostgreSQL container...${NC}"
    
    # Remove existing container if it exists
    docker rm -f auth-postgres 2>/dev/null || true
    
    # Start PostgreSQL container
    docker run -d \
        --name auth-postgres \
        -e POSTGRES_DB=auth_db \
        -e POSTGRES_USER=auth_user \
        -e POSTGRES_PASSWORD=auth_password123 \
        -p 5432:5432 \
        postgres:15-alpine
    
    # Wait for PostgreSQL to be ready
    wait_for_service "PostgreSQL" 5432
    
    # Get container IP and port
    POSTGRES_HOST="localhost"
    POSTGRES_PORT="5432"
    POSTGRES_DB="auth_db"
    POSTGRES_USER="auth_user"
    POSTGRES_PASSWORD="auth_password123"
    
    echo -e "${GREEN}✅ PostgreSQL started successfully${NC}"
    echo "   Host: $POSTGRES_HOST"
    echo "   Port: $POSTGRES_PORT"
    echo "   Database: $POSTGRES_DB"
    echo "   User: $POSTGRES_USER"
}

# Function to start Redis
start_redis() {
    echo -e "${YELLOW}🔴 Starting Redis container...${NC}"
    
    # Remove existing container if it exists
    docker rm -f auth-redis 2>/dev/null || true
    
    # Start Redis container
    docker run -d \
        --name auth-redis \
        -p 6379:6379 \
        redis:7-alpine redis-server --requirepass redis_password123
    
    # Wait for Redis to be ready
    wait_for_service "Redis" 6379
    
    # Get container details
    REDIS_HOST="localhost"
    REDIS_PORT="6379"
    REDIS_PASSWORD="redis_password123"
    
    echo -e "${GREEN}✅ Redis started successfully${NC}"
    echo "   Host: $REDIS_HOST"
    echo "   Port: $REDIS_PORT"
}

# Function to start RabbitMQ
start_rabbitmq() {
    echo -e "${YELLOW}🐰 Starting RabbitMQ container...${NC}"
    
    # Remove existing container if it exists
    docker rm -f auth-rabbitmq 2>/dev/null || true
    
    # Start RabbitMQ container
    docker run -d \
        --name auth-rabbitmq \
        -e RABBITMQ_DEFAULT_USER=rabbit_user \
        -e RABBITMQ_DEFAULT_PASS=rabbit_password123 \
        -p 5672:5672 \
        -p 15672:15672 \
        rabbitmq:3-management-alpine
    
    # Wait for RabbitMQ to be ready
    wait_for_service "RabbitMQ" 5672
    
    # Get container details
    RABBITMQ_HOST="localhost"
    RABBITMQ_PORT="5672"
    RABBITMQ_USER="rabbit_user"
    RABBITMQ_PASSWORD="rabbit_password123"
    
    echo -e "${GREEN}✅ RabbitMQ started successfully${NC}"
    echo "   Host: $RABBITMQ_HOST"
    echo "   Port: $RABBITMQ_PORT"
    echo "   Management UI: http://localhost:15672"
    echo "   User: $RABBITMQ_USER"
}

# Function to update .env file
update_env_file() {
    echo -e "${YELLOW}📝 Updating .env file...${NC}"
    
    # Create .env file with all configurations
    cat > .env << EOF
# Database Configuration
DATABASE_URL=postgresql://${POSTGRES_USER}:${POSTGRES_PASSWORD}@${POSTGRES_HOST}:${POSTGRES_PORT}/${POSTGRES_DB}
DB_MAX_CONNECTIONS=10
DB_MIN_CONNECTIONS=2
DB_CONNECT_TIMEOUT=30
DB_IDLE_TIMEOUT=600

# Redis Configuration
REDIS_URL=redis://:${REDIS_PASSWORD}@${REDIS_HOST}:${REDIS_PORT}
REDIS_MAX_CONNECTIONS=10
REDIS_CONNECT_TIMEOUT=5
REDIS_IDLE_TIMEOUT=300
REDIS_POOL_TIMEOUT=10

# JWT Configuration
JWT_SECRET=your-super-secret-jwt-key-change-this-in-production-$(openssl rand -hex 32)
JWT_ACCESS_TOKEN_EXPIRY=900
JWT_REFRESH_TOKEN_EXPIRY=604800
JWT_ISSUER=auth-service
JWT_AUDIENCE=borough-platform

# OTP Configuration
OTP_LENGTH=6
OTP_EXPIRY_SECONDS=300
OTP_MAX_ATTEMPTS=3
OTP_RATE_LIMIT_WINDOW=3600
OTP_MAX_REQUESTS_PER_WINDOW=5

# Server Configuration
SERVER_HOST=0.0.0.0
SERVER_PORT=8001
SERVER_WORKERS=4
SERVER_KEEP_ALIVE=75
SERVER_CLIENT_TIMEOUT=5000
SERVER_CLIENT_SHUTDOWN=5000

# Messaging Configuration
RABBITMQ_URL=amqp://${RABBITMQ_USER}:${RABBITMQ_PASSWORD}@${RABBITMQ_HOST}:${RABBITMQ_PORT}
MESSAGING_EXCHANGE_NAME=auth_events
MESSAGING_QUEUE_NAME=auth_notifications
MESSAGING_ROUTING_KEY=auth.notifications
MESSAGING_CONNECTION_TIMEOUT=30
MESSAGING_HEARTBEAT=60

# Logging
RUST_LOG=info,auth_service=debug

# Development
ENVIRONMENT=development
EOF

    echo -e "${GREEN}✅ .env file updated successfully${NC}"
}

# Function to show service status
show_status() {
    echo -e "\n${GREEN}🎉 All services are running!${NC}"
    echo -e "\n📋 Service Details:"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo -e "${YELLOW}PostgreSQL:${NC}"
    echo "  • Container: auth-postgres"
    echo "  • URL: postgresql://auth_user:auth_password123@localhost:5432/auth_db"
    echo "  • Connect: docker exec -it auth-postgres psql -U auth_user -d auth_db"
    echo ""
    echo -e "${YELLOW}Redis:${NC}"
    echo "  • Container: auth-redis"
    echo "  • URL: redis://:redis_password123@localhost:6379"
    echo "  • Connect: docker exec -it auth-redis redis-cli -a redis_password123"
    echo ""
    echo -e "${YELLOW}RabbitMQ:${NC}"
    echo "  • Container: auth-rabbitmq"
    echo "  • URL: amqp://rabbit_user:rabbit_password123@localhost:5672"
    echo "  • Management: http://localhost:15672 (rabbit_user/rabbit_password123)"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo ""
    echo -e "${GREEN}💡 Next steps:${NC}"
    echo "1. Run your database migrations: cargo run --bin migrate"
    echo "2. Start the auth service: cargo run"
    echo ""
    echo -e "${YELLOW}🛑 To stop all services:${NC}"
    echo "   docker stop auth-postgres auth-redis auth-rabbitmq"
    echo "   docker rm auth-postgres auth-redis auth-rabbitmq"
}

# Main execution
main() {
    echo "🏗️  Borough Auth Service - Development Environment Setup"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    
    check_docker
    
    # Start all services
    start_postgres
    start_redis
    start_rabbitmq
    
    # Update environment file
    update_env_file
    
    # Show final status
    show_status
}

# Run main function
main
