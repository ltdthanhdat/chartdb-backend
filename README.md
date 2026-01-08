# ChartDB Backend

Backend API sync service for ChartDB, written in Rust.

## Overview

This is the Rust backend that provides REST API endpoints for syncing ChartDB diagrams across devices. It uses PostgreSQL for persistent storage and Axum for the web framework.

## Quick Start

### Docker (Recommended)
```bash
# From project root
docker-compose up -d

# Backend will run on http://localhost:3000
```

### Local Development
```bash
# 1. Setup database
createdb chartdb

# 2. Set environment variables
echo "DATABASE_URL=postgresql://postgres:postgres@localhost:5432/chartdb" > .env

# 3. Run migrations
cargo install sqlx-cli
sqlx migrate run

# 4. Start server
cargo run --release
```

Server runs on `http://localhost:3000`

## API Endpoints

- `POST /api/sync/push` - Push diagram to server
- `GET /api/sync/pull/:id` - Pull diagram from server
- `GET /api/sync/diagrams` - List all diagrams
- `GET /health` - Health check

## Tech Stack

- **Framework:** Axum (async web framework)
- **Database:** PostgreSQL with SQLx (compile-time checked queries)
- **Serialization:** Serde (JSON)
- **CORS:** Tower-HTTP

## Project Structure

```
chartdb-backend/
├── src/
│   ├── main.rs          # Entry point, server setup
│   ├── handlers.rs      # API request handlers
│   ├── models.rs        # Data structures
│   └── routes.rs        # Route definitions
├── migrations/
│   ├── 001_init.sql            # Initial schema
│   └── 002_change_id_to_text.sql  # ID type migration
├── Cargo.toml           # Rust dependencies
├── Dockerfile           # Docker image
└── README.md            # This file
```

## Database Schema

### Tables

**diagrams**
- `id` (TEXT PRIMARY KEY)
- `name` (TEXT)
- `database_type` (TEXT)
- `database_edition` (TEXT)
- `created_at` (TIMESTAMPTZ)
- `updated_at` (TIMESTAMPTZ)

**tables**
- `id` (TEXT PRIMARY KEY)
- `diagram_id` (TEXT, FK)
- `name`, `schema`, `x`, `y`, `color`, etc.
- Full table definition in JSON

**relationships**
- `id` (TEXT PRIMARY KEY)
- `diagram_id` (TEXT, FK)
- Foreign key relationship data

And similar tables for: `dependencies`, `areas`, `custom_types`, `notes`

## Development

```bash
# Run with hot reload (cargo-watch)
cargo install cargo-watch
cargo watch -x run

# Run tests
cargo test

# Check code
cargo clippy

# Format code
cargo fmt

# Build release
cargo build --release
```

## Environment Variables

```env
DATABASE_URL=postgresql://postgres:postgres@localhost:5432/chartdb
PORT=3000
RUST_LOG=info  # debug, info, warn, error
```

## Migrations

Migrations are in `migrations/` directory and run automatically on startup.

To create a new migration:
```bash
# Create migration file
touch migrations/003_my_migration.sql

# Add SQL
echo "ALTER TABLE diagrams ADD COLUMN description TEXT;" > migrations/003_my_migration.sql
```

## API Response Format

All endpoints return JSON.

**Success:**
```json
{
  "id": "abc123",
  "name": "My Database",
  ...
}
```

**Error:**
```json
{
  "error": "Error message here"
}
```

## Troubleshooting

**Port already in use:**
```bash
# Find process
lsof -i :3000

# Kill it
kill -9 <PID>
```

**Database connection failed:**
```bash
# Check PostgreSQL is running
pg_isready

# Check connection string
echo $DATABASE_URL
```

**Migration errors:**
```bash
# Reset database
dropdb chartdb
createdb chartdb
sqlx migrate run
```

## Performance

- Uses connection pooling (10 connections by default)
- Transactions for data consistency
- Bulk operations for efficiency
- Async/await for concurrency

## Security Notes

Current implementation is for development only:
- No authentication
- No rate limiting
- CORS allows all origins

For production, add:
- JWT/OAuth authentication
- API keys
- Rate limiting
- HTTPS only
- Input validation
- Audit logging
