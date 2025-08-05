@echo off

REM status-services.bat - Script to check the status of all auth service containers

echo 📊 Auth Service Container Status
echo ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

REM Function to check container status
call :check_container "auth-postgres" "PostgreSQL" "5433"
call :check_container "auth-redis" "Redis" "6379"
call :check_container "auth-rabbitmq" "RabbitMQ" "5672"

echo 🔗 Quick Connection Commands:
echo ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
echo PostgreSQL: docker exec -it auth-postgres psql -U auth_user -d auth_db
echo Redis:      docker exec -it auth-redis redis-cli -a redis_password123
echo RabbitMQ:   http://localhost:15672 (rabbit_user/rabbit_password123)
echo.
echo 📋 View Logs:
echo ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
echo PostgreSQL:   docker logs -f auth-postgres
echo Redis:        docker logs -f auth-redis
echo RabbitMQ:     docker logs -f auth-rabbitmq

goto :eof

REM Function to check container status
:check_container
set container_name=%~1
set service_name=%~2
set port=%~3

docker ps --format "table {{.Names}}" | find "%container_name%" >nul 2>&1
if %errorlevel% equ 0 (
    echo ✅ %service_name%: Running - Port %port%
    
    REM Check if port is accessible
    netstat -an | find ":%port% " | find "LISTENING" >nul 2>&1
    if %errorlevel% equ 0 (
        echo    🔗 Port %port% is accessible
    ) else (
        echo    ⚠️  Port %port% is not accessible yet
    )
) else (
    echo ❌ %service_name%: Not running
)
echo.
exit /b 0
