#!/bin/bash

# Start infrastructure services

echo "Starting infrastructure services..."

cd infra
docker-compose up -d

echo "Infrastructure services started!"
echo ""
echo "Services available at:"
echo "- PostgreSQL: localhost:5432"
echo "- Redis: localhost:6379"
echo "- RabbitMQ Management: http://localhost:15672 (admin/admin)"
echo "- Meilisearch: http://localhost:7700"
echo "- Traefik Dashboard: http://localhost:8080"
echo "- Jaeger UI: http://localhost:16686"
echo "- Prometheus: http://localhost:9090"
echo "- Grafana: http://localhost:3000 (admin/admin)"
