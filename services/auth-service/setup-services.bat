@echo off
setlocal enabledelayedexpansion

REM setup-services.bat - Script to spin up required services for auth service
REM This script starts PostgreSQL, Redis, and RabbitMQ containers and updates .env file

echo 🚀 Starting required services for auth service...

REM Function to check if docker is running
echo Checking Docker status...
docker info >nul 2>&1
if %errorlevel% neq 0 (
    echo ❌ Docker is not running. Please start Docker and try again.
    exit /b 1
)
echo ✅ Docker is running

REM Start PostgreSQL
echo 🐘 Starting PostgreSQL container...
docker rm -f auth-postgres >nul 2>&1
docker run -d --name auth-postgres -e POSTGRES_DB=auth_db -e POSTGRES_USER=auth_user -e POSTGRES_PASSWORD=auth_password123 -p 5433:5432 postgres:15-alpine

REM Wait for PostgreSQL
echo ⏳ Waiting for PostgreSQL to be ready...
call :wait_for_service "PostgreSQL" 5433
if %errorlevel% neq 0 (
    echo ❌ PostgreSQL failed to start
    exit /b 1
)
echo ✅ PostgreSQL started successfully

REM Start Redis
echo 🔴 Starting Redis container...
docker rm -f auth-redis >nul 2>&1
docker run -d --name auth-redis -p 6379:6379 redis:7-alpine redis-server --requirepass redis_password123

REM Wait for Redis
echo ⏳ Waiting for Redis to be ready...
call :wait_for_service "Redis" 6379
if %errorlevel% neq 0 (
    echo ❌ Redis failed to start
    exit /b 1
)
echo ✅ Redis started successfully

REM Start RabbitMQ
echo 🐰 Starting RabbitMQ container...
docker rm -f auth-rabbitmq >nul 2>&1
docker run -d --name auth-rabbitmq -e RABBITMQ_DEFAULT_USER=rabbit_user -e RABBITMQ_DEFAULT_PASS=rabbit_password123 -p 5672:5672 -p 15672:15672 rabbitmq:3-management-alpine

REM Wait for RabbitMQ
echo ⏳ Waiting for RabbitMQ to be ready...
call :wait_for_service "RabbitMQ" 5672
if %errorlevel% neq 0 (
    echo ❌ RabbitMQ failed to start
    exit /b 1
)
echo ✅ RabbitMQ started successfully

REM Generate JWT secret
for /f %%i in ('powershell -command "[System.Web.Security.Membership]::GeneratePassword(64, 10)"') do set JWT_SECRET=%%i

REM Update .env file
echo 📝 Updating .env file...
(
echo # Database Configuration
echo DATABASE_URL=postgresql://auth_user:auth_password123@localhost:5433/auth_db
echo DB_MAX_CONNECTIONS=10
echo DB_MIN_CONNECTIONS=2
echo DB_CONNECT_TIMEOUT=30
echo DB_IDLE_TIMEOUT=600
echo.
echo # Redis Configuration
echo REDIS_URL=redis://:redis_password123@localhost:6379
echo REDIS_MAX_CONNECTIONS=10
echo REDIS_CONNECT_TIMEOUT=5
echo REDIS_IDLE_TIMEOUT=300
echo REDIS_POOL_TIMEOUT=10
echo.
echo # JWT Configuration
echo JWT_SECRET=!JWT_SECRET!
echo JWT_ACCESS_TOKEN_EXPIRY=900
echo JWT_REFRESH_TOKEN_EXPIRY=604800
echo JWT_ISSUER=auth-service
echo JWT_AUDIENCE=borough-platform
echo.
echo # OTP Configuration
echo OTP_LENGTH=6
echo OTP_EXPIRY_SECONDS=300
echo OTP_MAX_ATTEMPTS=3
echo OTP_RATE_LIMIT_WINDOW=3600
echo OTP_MAX_REQUESTS_PER_WINDOW=5
echo.
echo # Server Configuration
echo SERVER_HOST=localhost
echo SERVER_PORT=8001
echo SERVER_WORKERS=4
echo SERVER_KEEP_ALIVE=75
echo SERVER_CLIENT_TIMEOUT=5000
echo SERVER_CLIENT_SHUTDOWN=5000
echo.
echo # Messaging Configuration
echo RABBITMQ_URL=amqp://rabbit_user:rabbit_password123@localhost:5672
echo MESSAGING_EXCHANGE_NAME=auth_events
echo MESSAGING_QUEUE_NAME=auth_notifications
echo MESSAGING_ROUTING_KEY=auth.notifications
echo MESSAGING_CONNECTION_TIMEOUT=30
echo MESSAGING_HEARTBEAT=60
echo.
echo # Logging
echo RUST_LOG=info,auth_service=debug
echo.
echo # Development
echo ENVIRONMENT=development
) > .env

echo ✅ .env file updated successfully

REM Show status
echo.
echo 🎉 All services are running!
echo.
echo 📋 Service Details:
echo ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
echo PostgreSQL:
echo   • Container: auth-postgres
echo   • URL: postgresql://auth_user:auth_password123@localhost:5433/auth_db
echo   • Connect: docker exec -it auth-postgres psql -U auth_user -d auth_db
echo.
echo Redis:
echo   • Container: auth-redis
echo   • URL: redis://:redis_password123@localhost:6379
echo   • Connect: docker exec -it auth-redis redis-cli -a redis_password123
echo.
echo RabbitMQ:
echo   • Container: auth-rabbitmq
echo   • URL: amqp://rabbit_user:rabbit_password123@localhost:5672
echo   • Management: http://localhost:15672 (rabbit_user/rabbit_password123)
echo ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
echo.
echo 💡 Next steps:
echo 1. Run your database migrations: cargo run --bin migrate
echo 2. Start the auth service: cargo run
echo.
echo 🛑 To stop all services:
echo    docker stop auth-postgres auth-redis auth-rabbitmq
echo    docker rm auth-postgres auth-redis auth-rabbitmq

goto :eof

REM Function to check if port is open
:check_port
netstat -an | find ":%1 " | find "LISTENING" >nul
exit /b %errorlevel%

REM Function to wait for service (simplified for batch)
:wait_for_service
echo ⏳ Waiting for %~1 to be ready on port %2...
set /a attempts=0
:wait_loop
set /a attempts+=1
if %attempts% gtr 30 (
    echo ❌ %~1 failed to start within expected time
    exit /b 1
)
call :check_port %2
if %errorlevel% equ 0 (
    echo ✅ %~1 is ready!
    exit /b 0
)
timeout /t 2 /nobreak >nul
goto wait_loop