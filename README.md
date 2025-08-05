# Borough - Property Management Microservices Platform

A comprehensive property management platform built with Rust microservices architecture, featuring Clean Architecture principles, Actix-Web HTTP servers, and gRPC communication.

## 🏗 Architecture Overview

This project implements a microservices architecture using Rust, with each service following Clean Architecture principles. The system is designed to handle property management operations including user management, property listings, bookings, transactions, and more.

### 🎯 Clean Architecture Layers

Each microservice is structured with the following layers:

```
src/
├── application/    # Application Business Rules (Use Cases)
├── domain/         # Enterprise Business Rules (Entities & Domain Services)
├── infrastructure/ # External Interface Implementation (DB, APIs, etc.)
├── interface/      # Interface Adapters (HTTP Controllers, gRPC)
├── config/         # Configuration Management
├── grpc/           # gRPC Service Implementations
└── cache/          # Caching Layer (Redis)
```

## 📁 Project Structure

```
borough/
├── services/              # All microservices
│   ├── auth-service/      # Authentication & Authorization
│   ├── user-service/      # User Management
│   ├── property-service/  # Property Management
│   ├── booking-service/   # Booking Management
│   ├── transaction-service/ # Payment & Transactions
│   ├── notification-service/ # Notifications
│   ├── feedback-service/  # Reviews & Feedback
│   ├── search-service/    # Search & Filtering
│   └── external-comm-service/ # External API Integration
├── shared/               # Shared utilities and models
│   ├── src/
│   │   ├── models/       # Common data models
│   │   ├── events/       # Event definitions for messaging
│   │   ├── utils/        # Utility functions (JWT, hashing, etc.)
│   │   ├── config/       # Configuration management
│   │   ├── grpc_clients/ # Generated gRPC clients
│   │   └── observability/ # Tracing & monitoring
├── infra/                # Infrastructure setup
│   └── docker-compose.yml # Infrastructure services
├── observability/        # Monitoring configuration
│   ├── prometheus.yml    # Prometheus configuration
│   └── jaeger.yml       # Jaeger configuration
└── scripts/             # Automation scripts
    ├── build-all.ps1    # Build all services (Windows)
    ├── build-all.sh     # Build all services (Unix)
    ├── start-infra.ps1  # Start infrastructure (Windows)
    └── start-infra.sh   # Start infrastructure (Unix)
```

## 🚀 Services Overview

### Core Services

| Service                   | Port | gRPC Port | Description                                      |
| ------------------------- | ---- | --------- | ------------------------------------------------ |
| **auth-service**          | 8001 | 9001      | Authentication, OTP verification, JWT management |
| **user-service**          | 8002 | 9002      | User profiles, homeowner/guest management        |
| **property-service**      | 8003 | 9003      | Property listings, amenities, pricing            |
| **booking-service**       | 8004 | 9004      | Reservations, availability, booking management   |
| **transaction-service**   | 8005 | 9005      | Payments, refunds, financial transactions        |
| **notification-service**  | 8006 | 9006      | Email, SMS, push notifications                   |
| **feedback-service**      | 8007 | 9007      | Reviews, ratings, feedback management            |
| **search-service**        | 8008 | 9008      | Property search, filtering, recommendations      |
| **external-comm-service** | 8009 | 9009      | External API integrations (OTP, payments)        |

### Infrastructure Services

| Service         | Port          | Description                      |
| --------------- | ------------- | -------------------------------- |
| **PostgreSQL**  | 5432          | Primary database                 |
| **Redis**       | 6379          | Caching and session storage      |
| **RabbitMQ**    | 5672, 15672   | Message broker and management UI |
| **Meilisearch** | 7700          | Search engine                    |
| **Traefik**     | 80, 443, 8080 | Reverse proxy and load balancer  |
| **Jaeger**      | 16686         | Distributed tracing              |
| **Prometheus**  | 9090          | Metrics collection               |
| **Grafana**     | 3000          | Monitoring dashboards            |

## 🛠 Technology Stack

### Core Technologies

- **Language**: Rust 1.75+
- **Web Framework**: Actix-Web 4.4
- **gRPC**: Tonic 0.12
- **Database**: PostgreSQL with SQLx
- **Cache**: Redis
- **Message Queue**: RabbitMQ
- **Search**: Meilisearch
- **Observability**: OpenTelemetry, Jaeger, Prometheus

### Key Dependencies

- **Serialization**: Serde
- **Async Runtime**: Tokio
- **Security**: JWT, bcrypt
- **Configuration**: Config crate with environment variables
- **Logging**: env_logger, tracing

## 🚀 Getting Started

### Prerequisites

- Rust 1.75 or later
- Docker and Docker Compose
- PostgreSQL (optional, can use Docker)

### 1. Clone the Repository

```bash
git clone <repository-url>
cd borough
```

### 2. Start Infrastructure Services

**Windows (PowerShell):**

```powershell
.\scripts\start-infra.ps1
```

**Linux/macOS:**

```bash
chmod +x scripts/start-infra.sh
./scripts/start-infra.sh
```

### 3. Build All Services

**Windows (PowerShell):**

```powershell
.\scripts\build-all.ps1
```

**Linux/macOS:**

```bash
chmod +x scripts/build-all.sh
./scripts/build-all.sh
```

### 4. Run Individual Services

```bash
# Example: Running auth service
cd services/auth-service
cargo run

# Or with environment variables
HOST=127.0.0.1 PORT=8001 GRPC_PORT=9001 cargo run
```

## 🔧 Configuration

Each service can be configured using environment variables or configuration files:

### Environment Variables

```bash
# Service Configuration
HOST=127.0.0.1
PORT=8001
GRPC_PORT=9001

# Database
DATABASE_URL=postgresql://borough_user:borough_pass@localhost:5432/borough_db

# Redis
REDIS_URL=redis://localhost:6379

# RabbitMQ
RABBITMQ_URL=amqp://borough_user:borough_pass@localhost:5672

# JWT
JWT_SECRET=your-super-secret-key
JWT_EXPIRY_HOURS=24

# Logging
RUST_LOG=info
```

## 🧪 Health Checks

All services expose health check endpoints:

```bash
# Check auth service
curl http://localhost:8001/health

# Check user service
curl http://localhost:8002/health

# Check property service
curl http://localhost:8003/health
```

Expected response:

```json
{
  "status": "OK",
  "timestamp": "2024-01-15T10:30:00Z",
  "service": "auth-service"
}
```

## 📊 Monitoring & Observability

### Access Monitoring Tools

- **Jaeger Tracing**: http://localhost:16686
- **Prometheus Metrics**: http://localhost:9090
- **Grafana Dashboards**: http://localhost:3000 (admin/admin)
- **RabbitMQ Management**: http://localhost:15672 (borough_user/borough_pass)

### Service Discovery

Traefik automatically discovers services and provides load balancing. Access the Traefik dashboard at http://localhost:8080

## 🗄 Database Migrations

Each service includes SQLx migrations in the `migrations/` directory:

```bash
# Run migrations for a specific service
cd services/auth-service
sqlx migrate run

# Create a new migration
sqlx migrate add create_users_table
```

## 🔄 Event-Driven Architecture

Services communicate via RabbitMQ events defined in `shared/src/events.rs`:

- **UserCreatedEvent**: Triggered when a new user registers
- **PropertyCreatedEvent**: Triggered when a property is listed
- **BookingCreatedEvent**: Triggered when a booking is made
- **PaymentProcessedEvent**: Triggered when payment is completed
- **NotificationRequestEvent**: Triggered when notification is needed

## 🐳 Docker Support

Each service includes a multi-stage Dockerfile for production deployment:

```bash
# Build service Docker image
cd services/auth-service
docker build -t borough/auth-service .

# Run with Docker
docker run -p 8001:8001 -p 9001:9001 borough/auth-service
```

## 🧱 Development Workflow

### Adding a New Feature

1. **Domain Layer**: Define entities and business rules
2. **Application Layer**: Implement use cases
3. **Infrastructure Layer**: Add database repositories, external APIs
4. **Interface Layer**: Create HTTP endpoints and gRPC services
5. **Tests**: Add integration tests
6. **Events**: Define events for inter-service communication

### Service Communication

- **Synchronous**: gRPC for real-time operations
- **Asynchronous**: RabbitMQ events for eventual consistency
- **Caching**: Redis for frequently accessed data

## 📋 API Endpoints

### Auth Service (Port 8001)

- `GET /health` - Health check
- `POST /auth/login` - User login
- `POST /auth/register` - User registration
- `POST /auth/verify-otp` - OTP verification

### User Service (Port 8002)

- `GET /health` - Health check
- `GET /users/{id}` - Get user profile
- `PUT /users/{id}` - Update user profile
- `POST /users` - Create user

### Property Service (Port 8003)

- `GET /health` - Health check
- `GET /properties` - List properties
- `POST /properties` - Create property
- `GET /properties/{id}` - Get property details

### Booking Service (Port 8004)

- `GET /health` - Health check
- `POST /bookings` - Create booking
- `GET /bookings` - List bookings
- `GET /bookings/{id}` - Get booking details

## 🔐 Security

- **JWT Authentication**: Stateless authentication across services
- **Password Hashing**: bcrypt for secure password storage
- **Input Validation**: Comprehensive request validation
- **Rate Limiting**: Planned via Traefik middleware
- **HTTPS**: Production deployment with SSL certificates

## 🚀 Deployment

### Production Deployment

1. **Build Docker Images**: Use multi-stage Dockerfiles
2. **Environment Configuration**: Set production environment variables
3. **Database Setup**: Run migrations on production database
4. **Load Balancing**: Configure Traefik for production
5. **Monitoring**: Set up Prometheus and Grafana dashboards

### Kubernetes Deployment

(Future enhancement - Kubernetes manifests to be added)

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Follow Clean Architecture principles
4. Add comprehensive tests
5. Update documentation
6. Submit a pull request

## 📝 License

This project is licensed under the MIT License - see the LICENSE file for details.

## 🆘 Troubleshooting

### Common Issues

1. **Port Conflicts**: Ensure ports 8001-8009 and 9001-9009 are available
2. **Database Connection**: Verify PostgreSQL is running and accessible
3. **Redis Connection**: Check Redis service status
4. **Build Errors**: Ensure Rust 1.75+ is installed

### Getting Help

- Check service logs for detailed error messages
- Verify environment variable configuration
- Ensure all infrastructure services are running
- Review the health check endpoints

---

**Note**: This is a foundational setup. Business logic implementation will be built on top of this architecture as the project evolves.
