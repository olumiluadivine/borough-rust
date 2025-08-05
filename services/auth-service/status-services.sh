#!/bin/bash

# status-services.sh - Script to check the status of all auth service containers

echo "📊 Auth Service Container Status"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Function to check container status
check_container() {
    local container_name=$1
    local service_name=$2
    local port=$3
    
    if docker ps --format "table {{.Names}}\t{{.Status}}" | grep -q "^$container_name"; then
        local status=$(docker ps --format "table {{.Names}}\t{{.Status}}" | grep "^$container_name" | awk '{print $2, $3}')
        echo -e "${GREEN}✅ $service_name:${NC} Running ($status) - Port $port"
        
        # Check if port is accessible
        if nc -z localhost $port 2>/dev/null; then
            echo -e "   ${GREEN}🔗 Port $port is accessible${NC}"
        else
            echo -e "   ${YELLOW}⚠️  Port $port is not accessible yet${NC}"
        fi
    else
        echo -e "${RED}❌ $service_name:${NC} Not running"
    fi
    echo ""
}

# Check each service
check_container "auth-postgres" "PostgreSQL" "5432"
check_container "auth-redis" "Redis" "6379"
check_container "auth-rabbitmq" "RabbitMQ" "5672"

# Show quick connection commands
echo "🔗 Quick Connection Commands:"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "PostgreSQL: docker exec -it auth-postgres psql -U auth_user -d auth_db"
echo "Redis:      docker exec -it auth-redis redis-cli -a redis_password123"
echo "RabbitMQ:   http://localhost:15672 (rabbit_user/rabbit_password123)"
echo ""

# Show logs command
echo "📋 View Logs:"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "All services: docker logs auth-postgres && docker logs auth-redis && docker logs auth-rabbitmq"
echo "PostgreSQL:   docker logs -f auth-postgres"
echo "Redis:        docker logs -f auth-redis"
echo "RabbitMQ:     docker logs -f auth-rabbitmq"
