@echo off

REM stop-services.bat - Script to stop and remove all auth service containers

echo 🛑 Stopping auth service containers...

REM Function to stop and remove container
call :stop_container "auth-postgres" "PostgreSQL"
call :stop_container "auth-redis" "Redis"
call :stop_container "auth-rabbitmq" "RabbitMQ"

REM Clean up any dangling volumes
echo 🧹 Cleaning up unused volumes...
docker volume prune -f >nul 2>&1

echo.
echo 🎉 All auth service containers have been stopped and removed!
echo.
echo 💡 Note: Your .env file is preserved. To restart services, run:
echo    setup-services.bat

goto :eof

REM Function to stop and remove container
:stop_container
set container_name=%~1
set service_name=%~2

docker ps -a --format "table {{.Names}}" | find "%container_name%" >nul 2>&1
if %errorlevel% equ 0 (
    echo 🔄 Stopping %service_name% container...
    docker stop %container_name% >nul 2>&1
    docker rm %container_name% >nul 2>&1
    echo ✅ %service_name% stopped and removed
) else (
    echo ℹ️  %service_name% container not found
)
exit /b 0
