# ChartDB Backend

Backend API cho ChartDB sync service, viết bằng Rust.

## Features

- **Push**: Sync diagram từ IndexedDB lên PostgreSQL
- **Pull**: Lấy diagram từ PostgreSQL về IndexedDB
- Simple & Fast: Chỉ 2 endpoints, không phức tạp

## Setup

### Local Development

1. Install dependencies:
```bash
cargo install sqlx-cli
```

2. Setup database:
```bash
createdb chartdb
```

3. Run migrations:
```bash
sqlx migrate run
```

4. Run server:
```bash
cargo run
```

### Docker

```bash
docker-compose up
```

Server sẽ chạy trên `http://localhost:3000`

## API Endpoints

### POST /api/sync/push

Push diagram từ IndexedDB lên server.

**Request:**
```json
{
  "diagram": {
    "id": "uuid",
    "name": "My Database",
    "databaseType": "postgresql",
    "tables": [...],
    "relationships": [...]
  }
}
```

**Response:**
```json
{
  "success": true,
  "diagramId": "uuid"
}
```

### GET /api/sync/pull/:id

Pull diagram từ server về.

**Response:**
```json
{
  "id": "uuid",
  "name": "My Database",
  "tables": [...],
  "relationships": [...]
}
```

### GET /health

Health check endpoint.

## Environment Variables

- `DATABASE_URL`: PostgreSQL connection string
- `PORT`: Server port (default: 3000)

