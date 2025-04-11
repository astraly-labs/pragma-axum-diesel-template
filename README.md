# Pragma Template

Pragma Template is a template for creating a new service using axum and diesel. It follows modern Rust practices, emphasizing reliability, safety, and maintainability.

## Features

- **Modern Architecture**: Built with a modular crate-based design
- **High Performance**: Leverages Rust's performance and safe concurrency
- **Database Integration**: Uses PostgreSQL with Diesel ORM and connection pooling
- **API Documentation**: Auto-generated OpenAPI documentation via Utoipa
- **Observability**: Integrated with OpenTelemetry for tracing and monitoring
- **Containerization**: Docker ready with multi-stage builds for minimal image size
- **Development Tools**: Docker Compose for local development environment
- **CI/CD**: GitHub Actions workflows for automated testing and deployments

## Project Structure

The project is organized into a workspace with multiple crates:

```
pragma-axum-diesel-template/
├── bin/                     # Binary crates
│   └── pragma-bin/          # Main executable
├── crates/                  # Library crates
│   ├── pragma-api/          # API logic and routes
│   └── pragma-db/           # Database models and interactions
|       └── migrations/      # Database migrations
└── src/                     # Workspace shared code
```

This structure allows for clear separation of concerns and easier maintenance.

## Getting Started

### Prerequisites

- Rust 1.86.0 or newer
- Docker and Docker Compose (for development environment)
- PostgreSQL (for production or standalone use)

### Environment Setup

1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/pragma-axum-diesel-template.git
   cd pragma-axum-diesel-template
   ```

2. Copy the example environment file:
   ```bash
   cp .env.example .env
   ```

3. Update `.env` with your configuration values

### Development Environment

The project includes a Docker Compose file for setting up a local development environment:

```bash
# Start the development environment
docker-compose -f compose.dev.yaml up -d

# Check the services
docker-compose -f compose.dev.yaml ps
```

This will start:
- PostgreSQL database
- Grafana LGTM stack for observability (Loki, Grafana, Tempo, Mimir)

### Building the Project

```bash
# Build the project
cargo build

# Run tests
cargo nextest run

# Run with cargo
cargo run --bin pragma-bin
```

### Running in Docker

```bash
# Build the Docker image
docker build -t pragma-bin .

# Run the container
docker run -p 3000:3000 --env-file .env pragma-bin
```

## Configuration

The service is configured via environment variables:

- `DATABASE_URL`: PostgreSQL connection string
- `DATABASE_MAX_CONN`: Maximum database connections in the pool
- `API_PORT`: Port for the API server
- `OTEL_COLLECTOR_ENDPOINT`: OpenTelemetry collector endpoint for tracing

## API Documentation

When the service is running, API documentation is available at:

```
http://localhost:{API_PORT}/v1/docs
```

The documentation is automatically generated from the API code using Utoipa.

## Database Migrations

Database migrations are managed with Diesel:

```bash
# Install Diesel CLI (if not already installed)
cargo install diesel_cli --no-default-features --features postgres

# Run migrations
diesel migration run

# Create a new migration
diesel migration generate name_of_migration
```

## Testing

```bash
# Run all tests
cargo test

# Run specific tests
cargo test -p pragma-api

# Format and lint code
make format
```

## Project Completion Guide

To complete this template you can:

1. **Models and Business Logic**
   - Implement models in `crates/pragma-db/src/models/`
   - Add business logic in `crates/pragma-api/src/handlers/`
   - Create database migrations

2. **API Endpoints**
   - Implement endpoints in `crates/pragma-api/src/handlers/`

3. **Testing and Documentation**
   - Write comprehensive tests for all endpoints
   - Complete API documentation
   - Add usage examples

## Best Practices

- **Security**: Always follow security best practices
- **Testing**: Maintain high test coverage using codecov and nextes.st
- **Documentation**: Keep API documentation up to date
- **Error Handling**: Implement proper error handling and user feedback
- **Rate Limiting**: Consider implementing rate limiting to prevent abuse
- **Audit Logging**: Add detailed audit logging for security events

## Contributing

1. Fork the repository
2. Create your feature branch: `git checkout -b feature/amazing-feature`
3. Commit your changes: `git commit -m 'Add some amazing feature'`
4. Push to the branch: `git push origin feature/amazing-feature`
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgements

- [Rust](https://www.rust-lang.org/)
- [Axum](https://github.com/tokio-rs/axum)
- [Diesel](https://diesel.rs/)
- [OpenTelemetry](https://opentelemetry.io/)
- [Utoipa](https://github.com/juhaku/utoipa)