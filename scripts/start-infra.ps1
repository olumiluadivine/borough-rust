# PowerShell script to start infrastructure

Write-Host "Starting infrastructure services..." -ForegroundColor Green

Set-Location infra
docker-compose up -d

Write-Host "Infrastructure services started!" -ForegroundColor Green
Write-Host ""
Write-Host "Services available at:" -ForegroundColor Yellow
Write-Host "- PostgreSQL: localhost:5432"
Write-Host "- Redis: localhost:6379"
Write-Host "- RabbitMQ Management: http://localhost:15672 (borough_user/borough_pass)"
Write-Host "- Meilisearch: http://localhost:7700"
Write-Host "- Traefik Dashboard: http://localhost:8080"
Write-Host "- Jaeger UI: http://localhost:16686"
Write-Host "- Prometheus: http://localhost:9090"
Write-Host "- Grafana: http://localhost:3000 (admin/admin)"
