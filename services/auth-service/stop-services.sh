#!/bin/bash

# stop-services.sh - Script to stop and remove all auth service containers

set -e

echo "🛑 Stopping auth service containers..."

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Function to stop and remove container
stop_container() {
    local container_name=$1
    local service_name=$2
    
    if docker ps -a --format "table {{.Names}}" | grep -q "^$container_name$"; then
        echo -e "${YELLOW}🔄 Stopping $service_name container...${NC}"
        docker stop $container_name >/dev/null 2>&1 || true
        docker rm $container_name >/dev/null 2>&1 || true
        echo -e "${GREEN}✅ $service_name stopped and removed${NC}"
    else
        echo -e "${YELLOW}ℹ️  $service_name container not found${NC}"
    fi
}

# Stop all containers
stop_container "auth-postgres" "PostgreSQL"
stop_container "auth-redis" "Redis"
stop_container "auth-rabbitmq" "RabbitMQ"

# Clean up any dangling volumes (optional)
echo -e "${YELLOW}🧹 Cleaning up unused volumes...${NC}"
docker volume prune -f >/dev/null 2>&1 || true

echo -e "\n${GREEN}🎉 All auth service containers have been stopped and removed!${NC}"
echo -e "\n${YELLOW}💡 Note:${NC} Your .env file is preserved. To restart services, run:"
echo "   ./setup-services.sh (Linux/Mac)"
echo "   setup-services.bat (Windows)"
