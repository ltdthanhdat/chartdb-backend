# ChartDB Backend

Backend API sync service for ChartDB, written in Rust.

## Getting Started

### Docker (Recommended)

```bash
docker-compose up
```

### Local Development

```bash
# 1. Setup database
createdb chartdb

# 2. Run migrations
sqlx migrate run

# 3. Run server
cargo run
```

Server runs on `http://localhost:3000`

## Frontend Integration

### Push Flow (Frontend → Backend)

```mermaid
sequenceDiagram
    participant F as Frontend<br/>(React)
    participant IDB as IndexedDB<br/>(Local)
    participant B as Backend<br/>(Rust)
    participant PG as PostgreSQL

    F->>IDB: User edits → Save immediately
    F->>F: Debounce 3s
    F->>B: POST /api/sync/push<br/>{ diagram }
    B->>PG: BEGIN TRANSACTION
    B->>PG: DELETE old data
    B->>PG: INSERT new data
    B->>PG: COMMIT
    B-->>F: { success: true }
```

### Pull Flow (Backend → Frontend)

```mermaid
sequenceDiagram
    participant F as Frontend<br/>(React)
    participant B as Backend<br/>(Rust)
    participant PG as PostgreSQL
    participant IDB as IndexedDB<br/>(Local)

    F->>B: GET /api/sync/pull/:id
    
    alt Diagram exists on server
        B->>PG: SELECT diagram + tables + relationships
        PG-->>B: Full diagram data
        B-->>F: Diagram JSON
        F->>IDB: Update local cache
    else Not found on server
        B-->>F: 404 Not Found
        F->>IDB: Load from local
    end
    
    F-->>F: Display diagram
```

### Architecture Overview

```mermaid
graph LR
    A[Frontend<br/>React] -->|HTTP POST| B[Backend<br/>Rust/Axum]
    A -->|HTTP GET| B
    B -->|SQL| C[(PostgreSQL)]
    
    A -->|Local Cache| D[(IndexedDB)]
    D -.->|Sync| B
    
    style A fill:#e1f5ff
    style B fill:#fff4e1
    style C fill:#e1f5ff
    style D fill:#e1f5ff
```

## API Endpoints

- `POST /api/sync/push` - Frontend pushes diagram to server
- `GET /api/sync/pull/:id` - Frontend pulls diagram from server
- `GET /health` - Health check

## Environment Variables

```env
DATABASE_URL=postgresql://chartdb:chartdb@localhost:5432/chartdb
PORT=3000
```
