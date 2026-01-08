-- Create diagrams table
CREATE TABLE IF NOT EXISTS diagrams (
    id TEXT PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    database_type VARCHAR(50) NOT NULL,
    database_edition VARCHAR(50),
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    version INTEGER DEFAULT 1
);

-- Create db_tables table
CREATE TABLE IF NOT EXISTS db_tables (
    id TEXT PRIMARY KEY,
    diagram_id TEXT NOT NULL REFERENCES diagrams(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    schema VARCHAR(255),
    x DOUBLE PRECISION,
    y DOUBLE PRECISION,
    width DOUBLE PRECISION,
    color VARCHAR(50),
    comment TEXT,
    is_view BOOLEAN DEFAULT FALSE,
    is_materialized_view BOOLEAN DEFAULT FALSE,
    "order" INTEGER,
    fields JSONB NOT NULL,
    indexes JSONB DEFAULT '[]'::jsonb,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    version INTEGER DEFAULT 1
);

-- Create db_relationships table
CREATE TABLE IF NOT EXISTS db_relationships (
    id TEXT PRIMARY KEY,
    diagram_id TEXT NOT NULL REFERENCES diagrams(id) ON DELETE CASCADE,
    name VARCHAR(255),
    source_schema VARCHAR(255),
    source_table_id TEXT NOT NULL,
    target_schema VARCHAR(255),
    target_table_id TEXT NOT NULL,
    source_field_id TEXT,
    target_field_id TEXT,
    source_cardinality VARCHAR(20),
    target_cardinality VARCHAR(20),
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    version INTEGER DEFAULT 1
);

-- Create db_dependencies table
CREATE TABLE IF NOT EXISTS db_dependencies (
    id TEXT PRIMARY KEY,
    diagram_id TEXT NOT NULL REFERENCES diagrams(id) ON DELETE CASCADE,
    schema VARCHAR(255),
    table_id TEXT NOT NULL,
    dependent_schema VARCHAR(255),
    dependent_table_id TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT NOW(),
    version INTEGER DEFAULT 1
);

-- Create areas table
CREATE TABLE IF NOT EXISTS areas (
    id TEXT PRIMARY KEY,
    diagram_id TEXT NOT NULL REFERENCES diagrams(id) ON DELETE CASCADE,
    name VARCHAR(255),
    x DOUBLE PRECISION,
    y DOUBLE PRECISION,
    width DOUBLE PRECISION,
    height DOUBLE PRECISION,
    color VARCHAR(50),
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    version INTEGER DEFAULT 1
);

-- Create db_custom_types table
CREATE TABLE IF NOT EXISTS db_custom_types (
    id TEXT PRIMARY KEY,
    diagram_id TEXT NOT NULL REFERENCES diagrams(id) ON DELETE CASCADE,
    schema VARCHAR(255),
    type VARCHAR(255) NOT NULL,
    kind VARCHAR(50),
    values JSONB,
    fields JSONB,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    version INTEGER DEFAULT 1
);

-- Create notes table
CREATE TABLE IF NOT EXISTS notes (
    id TEXT PRIMARY KEY,
    diagram_id TEXT NOT NULL REFERENCES diagrams(id) ON DELETE CASCADE,
    content TEXT,
    x DOUBLE PRECISION,
    y DOUBLE PRECISION,
    width DOUBLE PRECISION,
    height DOUBLE PRECISION,
    color VARCHAR(50),
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    version INTEGER DEFAULT 1
);

-- Create indexes
CREATE INDEX IF NOT EXISTS idx_db_tables_diagram_id ON db_tables(diagram_id);
CREATE INDEX IF NOT EXISTS idx_db_relationships_diagram_id ON db_relationships(diagram_id);
CREATE INDEX IF NOT EXISTS idx_db_dependencies_diagram_id ON db_dependencies(diagram_id);
CREATE INDEX IF NOT EXISTS idx_areas_diagram_id ON areas(diagram_id);
CREATE INDEX IF NOT EXISTS idx_db_custom_types_diagram_id ON db_custom_types(diagram_id);
CREATE INDEX IF NOT EXISTS idx_notes_diagram_id ON notes(diagram_id);

