# Auth Service

The authentication and authorization service for the Borough property management platform.

## Features

- User authentication (login/logout)
- JWT token management with refresh tokens
- OTP verification (email and SMS)
- Password reset functionality
- Security questions for account recovery
- Rate limiting and security middleware
- Clean Architecture implementation
- PostgreSQL database integration
- Redis caching for sessions and OTP storage
- gRPC services for internal service communication

## Architecture

This service follows Clean Architecture principles with the following layers:

### Domain Layer (`src/domain/`)

- **Entities**: Core business objects (User, RefreshToken, SecurityQuestion)
- **Repositories**: Interfaces for data access
- **Services**: Domain business logic

### Application Layer (`src/application/`)

- **Use Cases**: Application-specific business rules
  - LoginUseCase
  - OtpUseCase
  - PasswordResetUseCase
  - SecurityQuestionUseCase
  - RefreshTokenUseCase

### Infrastructure Layer (`src/infrastructure/`)

- **Database**: PostgreSQL repository implementations
- **Messaging**: RabbitMQ event publishing
- **Config**: Environment-based configuration

### Interface Layer (`src/interface/`)

- **Controllers**: HTTP request handlers
- **Routes**: API endpoint definitions
- **Middleware**: Rate limiting, logging, authentication
- **gRPC**: Internal service communication

### Cache Layer (`src/cache/`)

- **AuthCacheService**: Session management
- **OtpCacheService**: OTP storage and rate limiting

## API Endpoints

### Authentication

- `POST /api/v1/auth/login` - User login
- `POST /api/v1/auth/logout` - User logout
- `POST /api/v1/auth/refresh` - Refresh access token

### OTP Management

- `POST /api/v1/otp/send/email` - Send OTP via email
- `POST /api/v1/otp/send/sms` - Send OTP via SMS
- `POST /api/v1/otp/verify` - Verify OTP code
- `POST /api/v1/otp/resend` - Resend OTP

### Password Management

- `POST /api/v1/password/reset/request` - Request password reset
- `POST /api/v1/password/reset/confirm` - Confirm password reset
- `PUT /api/v1/password/change` - Change password

### Security Questions

- `POST /api/v1/security-questions` - Create security questions
- `GET /api/v1/security-questions/user/{user_id}` - Get user questions
- `POST /api/v1/security-questions/verify` - Verify security answers
- `PUT /api/v1/security-questions` - Update security questions
- `DELETE /api/v1/security-questions/user/{user_id}` - Delete user questions

### Token Management

- `POST /api/v1/tokens/refresh` - Refresh access token
- `POST /api/v1/tokens/revoke` - Revoke refresh token
- `POST /api/v1/tokens/user/{user_id}/revoke-all` - Revoke all user tokens
- `GET /api/v1/tokens/user/{user_id}/active` - Get user active tokens

### Health Checks

- `GET /health` - Service health status
- `GET /health/ready` - Service readiness status

## Configuration

Copy `.env.example` to `.env` and configure the following:

### Database

- `DATABASE_URL`: PostgreSQL connection string
- `DB_MAX_CONNECTIONS`: Maximum database connections
- `DB_MIN_CONNECTIONS`: Minimum database connections

### Redis

- `REDIS_URL`: Redis connection string
- `REDIS_MAX_CONNECTIONS`: Maximum Redis connections

### JWT

- `JWT_SECRET`: Secret key for JWT signing
- `JWT_ACCESS_TOKEN_EXPIRY`: Access token expiry in seconds
- `JWT_REFRESH_TOKEN_EXPIRY`: Refresh token expiry in seconds

### OTP

- `OTP_LENGTH`: OTP code length (default: 6)
- `OTP_EXPIRY_SECONDS`: OTP expiry time (default: 300)
- `OTP_MAX_ATTEMPTS`: Maximum OTP verification attempts

### Server

- `SERVER_HOST`: Server bind address (default: 0.0.0.0)
- `SERVER_PORT`: HTTP server port (default: 8001)
- `SERVER_WORKERS`: Number of worker threads

### Messaging

- `RABBITMQ_URL`: RabbitMQ connection string
- `MESSAGING_EXCHANGE_NAME`: Exchange name for events

## Development

### Prerequisites

- Rust 1.75+
- PostgreSQL 14+
- Redis 6+
- RabbitMQ (for messaging)

### Setup

1. Install dependencies:

   ```bash
   cargo build
   ```

2. Set up environment variables:

   ```bash
   cp .env.example .env
   # Edit .env with your configuration
   ```

3. Run database migrations:

   ```bash
   sqlx migrate run
   ```

4. Start the service:
   ```bash
   cargo run
   ```

### Database Migrations

Database migrations should be created in the `migrations/` directory using SQLx:

```bash
sqlx migrate add create_users_table
sqlx migrate add create_refresh_tokens_table
sqlx migrate add create_security_questions_table
```

### Testing

Run tests with:

```bash
cargo test
```

## Security Features

- Password hashing with bcrypt
- JWT token validation
- Rate limiting per IP address
- Account lockout after failed attempts
- OTP rate limiting
- Security question validation
- Refresh token rotation
- Request logging with correlation IDs

## Dependencies

Key dependencies include:

- `actix-web`: HTTP web framework
- `sqlx`: Async PostgreSQL driver
- `redis`: Redis client
- `tokio`: Async runtime
- `serde`: Serialization framework
- `uuid`: UUID generation
- `chrono`: Date/time handling
- `bcrypt`: Password hashing
- `jsonwebtoken`: JWT handling
- `tonic`: gRPC framework

## Monitoring

The service includes:

- Health check endpoints
- Request/response logging
- Performance metrics
- Error tracking with correlation IDs
