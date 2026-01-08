use crate::models::{Diagram, ErrorResponse, PushRequest, PushResponse};
use axum::{extract::Path, extract::State, http::StatusCode, response::Json};
use sqlx::{PgPool, Row};

pub async fn push_diagram(
    State(pool): State<PgPool>,
    Json(payload): Json<PushRequest>,
) -> Result<Json<PushResponse>, (StatusCode, Json<ErrorResponse>)> {
    let diagram = payload.diagram;

    let mut tx = pool
        .begin()
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: format!("Database error: {}", e),
                }),
            )
        })?;

    // Upsert diagram
    sqlx::query(
        r#"
        INSERT INTO diagrams (id, name, database_type, database_edition, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, NOW())
        ON CONFLICT (id) DO UPDATE SET
            name = EXCLUDED.name,
            database_type = EXCLUDED.database_type,
            database_edition = EXCLUDED.database_edition,
            updated_at = NOW()
        "#,
    )
    .bind(&diagram.id)
    .bind(&diagram.name)
    .bind(&diagram.database_type)
    .bind(&diagram.database_edition)
    .bind(diagram.created_at)
    .execute(&mut *tx)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: format!("Failed to insert diagram: {}", e),
            }),
        )
    })?;

    // Delete old data
    sqlx::query("DELETE FROM db_tables WHERE diagram_id = $1")
        .bind(&diagram.id)
        .execute(&mut *tx)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: format!("Failed to delete tables: {}", e),
                }),
            )
        })?;

    sqlx::query("DELETE FROM db_relationships WHERE diagram_id = $1")
        .bind(&diagram.id)
        .execute(&mut *tx)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: format!("Failed to delete relationships: {}", e),
                }),
            )
        })?;

    sqlx::query("DELETE FROM db_dependencies WHERE diagram_id = $1")
        .bind(&diagram.id)
        .execute(&mut *tx)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: format!("Failed to delete dependencies: {}", e),
                }),
            )
        })?;

    sqlx::query("DELETE FROM areas WHERE diagram_id = $1")
        .bind(&diagram.id)
        .execute(&mut *tx)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: format!("Failed to delete areas: {}", e),
                }),
            )
        })?;

    sqlx::query("DELETE FROM db_custom_types WHERE diagram_id = $1")
        .bind(&diagram.id)
        .execute(&mut *tx)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: format!("Failed to delete custom types: {}", e),
                }),
            )
        })?;

    sqlx::query("DELETE FROM notes WHERE diagram_id = $1")
        .bind(&diagram.id)
        .execute(&mut *tx)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: format!("Failed to delete notes: {}", e),
                }),
            )
        })?;

    // Insert tables
    if let Some(tables) = diagram.tables {
        for table in tables {
            sqlx::query(
                r#"
                INSERT INTO db_tables (
                    id, diagram_id, name, schema, x, y, width, color, comment,
                    is_view, is_materialized_view, "order", fields, indexes
                ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
                "#,
            )
            .bind(&table.id)
            .bind(&table.diagram_id)
            .bind(&table.name)
            .bind(&table.schema)
            .bind(table.x)
            .bind(table.y)
            .bind(table.width)
            .bind(&table.color)
            .bind(&table.comment)
            .bind(table.is_view.unwrap_or(false))
            .bind(table.is_materialized_view.unwrap_or(false))
            .bind(table.order)
            .bind(&table.fields)
            .bind(&table.indexes)
            .execute(&mut *tx)
            .await
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ErrorResponse {
                        error: format!("Failed to insert table: {}", e),
                    }),
                )
            })?;
        }
    }

    // Insert relationships
    if let Some(relationships) = diagram.relationships {
        for rel in relationships {
            sqlx::query(
                r#"
                INSERT INTO db_relationships (
                    id, diagram_id, name, source_schema, source_table_id,
                    target_schema, target_table_id, source_field_id, target_field_id,
                    source_cardinality, target_cardinality
                ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
                "#,
            )
            .bind(&rel.id)
            .bind(&rel.diagram_id)
            .bind(&rel.name)
            .bind(&rel.source_schema)
            .bind(&rel.source_table_id)
            .bind(&rel.target_schema)
            .bind(&rel.target_table_id)
            .bind(&rel.source_field_id)
            .bind(&rel.target_field_id)
            .bind(&rel.source_cardinality)
            .bind(&rel.target_cardinality)
            .execute(&mut *tx)
            .await
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ErrorResponse {
                        error: format!("Failed to insert relationship: {}", e),
                    }),
                )
            })?;
        }
    }

    // Insert dependencies
    if let Some(dependencies) = diagram.dependencies {
        for dep in dependencies {
            sqlx::query(
                r#"
                INSERT INTO db_dependencies (
                    id, diagram_id, schema, table_id, dependent_schema, dependent_table_id
                ) VALUES ($1, $2, $3, $4, $5, $6)
                "#,
            )
            .bind(&dep.id)
            .bind(&dep.diagram_id)
            .bind(&dep.schema)
            .bind(&dep.table_id)
            .bind(&dep.dependent_schema)
            .bind(&dep.dependent_table_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ErrorResponse {
                        error: format!("Failed to insert dependency: {}", e),
                    }),
                )
            })?;
        }
    }

    // Insert areas
    if let Some(areas) = diagram.areas {
        for area in areas {
            sqlx::query(
                r#"
                INSERT INTO areas (
                    id, diagram_id, name, x, y, width, height, color
                ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
                "#,
            )
            .bind(&area.id)
            .bind(&area.diagram_id)
            .bind(&area.name)
            .bind(area.x)
            .bind(area.y)
            .bind(area.width)
            .bind(area.height)
            .bind(&area.color)
            .execute(&mut *tx)
            .await
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ErrorResponse {
                        error: format!("Failed to insert area: {}", e),
                    }),
                )
            })?;
        }
    }

    // Insert custom types
    if let Some(custom_types) = diagram.custom_types {
        for ct in custom_types {
            sqlx::query(
                r#"
                INSERT INTO db_custom_types (
                    id, diagram_id, schema, type, kind, values, fields
                ) VALUES ($1, $2, $3, $4, $5, $6, $7)
                "#,
            )
            .bind(&ct.id)
            .bind(&ct.diagram_id)
            .bind(&ct.schema)
            .bind(&ct.r#type)
            .bind(&ct.kind)
            .bind(&ct.values)
            .bind(&ct.fields)
            .execute(&mut *tx)
            .await
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ErrorResponse {
                        error: format!("Failed to insert custom type: {}", e),
                    }),
                )
            })?;
        }
    }

    // Insert notes
    if let Some(notes) = diagram.notes {
        for note in notes {
            sqlx::query(
                r#"
                INSERT INTO notes (
                    id, diagram_id, content, x, y, width, height, color
                ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
                "#,
            )
            .bind(&note.id)
            .bind(&note.diagram_id)
            .bind(&note.content)
            .bind(note.x)
            .bind(note.y)
            .bind(note.width)
            .bind(note.height)
            .bind(&note.color)
            .execute(&mut *tx)
            .await
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ErrorResponse {
                        error: format!("Failed to insert note: {}", e),
                    }),
                )
            })?;
        }
    }

    tx.commit()
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: format!("Failed to commit transaction: {}", e),
                }),
            )
        })?;

    Ok(Json(PushResponse {
        success: true,
        diagram_id: diagram.id,
    }))
}

pub async fn pull_diagram(
    State(pool): State<PgPool>,
    Path(id): Path<String>,
) -> Result<Json<Diagram>, (StatusCode, Json<ErrorResponse>)> {
    // Get diagram
    let diagram_row = sqlx::query("SELECT * FROM diagrams WHERE id = $1")
        .bind(&id)
        .fetch_optional(&pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: format!("Database error: {}", e),
                }),
            )
        })?;

    let diagram_row = match diagram_row {
        Some(row) => row,
        None => {
            return Err((
                StatusCode::NOT_FOUND,
                Json(ErrorResponse {
                    error: "Diagram not found".to_string(),
                }),
            ))
        }
    };

    let mut diagram = Diagram {
        id: diagram_row.get("id"),
        name: diagram_row.get("name"),
        database_type: diagram_row.get("database_type"),
        database_edition: diagram_row.get("database_edition"),
        created_at: diagram_row.get("created_at"),
        updated_at: diagram_row.get("updated_at"),
        tables: None,
        relationships: None,
        dependencies: None,
        areas: None,
        custom_types: None,
        notes: None,
    };

    // Get tables
    let table_rows = sqlx::query("SELECT * FROM db_tables WHERE diagram_id = $1")
        .bind(&id)
        .fetch_all(&pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: format!("Failed to fetch tables: {}", e),
                }),
            )
        })?;

    use crate::models::Table;
    let tables: Vec<Table> = table_rows
        .into_iter()
        .map(|row| Table {
            id: row.get("id"),
            diagram_id: row.get("diagram_id"),
            name: row.get("name"),
            schema: row.get("schema"),
            x: row.get("x"),
            y: row.get("y"),
            width: row.get("width"),
            color: row.get("color"),
            comment: row.get("comment"),
            is_view: row.get("is_view"),
            is_materialized_view: row.get("is_materialized_view"),
            order: row.get("order"),
            fields: row.get("fields"),
            indexes: row.get("indexes"),
        })
        .collect();

    diagram.tables = Some(tables);

    // Get relationships
    let rel_rows = sqlx::query("SELECT * FROM db_relationships WHERE diagram_id = $1")
        .bind(&id)
        .fetch_all(&pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: format!("Failed to fetch relationships: {}", e),
                }),
            )
        })?;

    use crate::models::Relationship;
    let relationships: Vec<Relationship> = rel_rows
        .into_iter()
        .map(|row| Relationship {
            id: row.get("id"),
            diagram_id: row.get("diagram_id"),
            name: row.get("name"),
            source_schema: row.get("source_schema"),
            source_table_id: row.get("source_table_id"),
            target_schema: row.get("target_schema"),
            target_table_id: row.get("target_table_id"),
            source_field_id: row.get("source_field_id"),
            target_field_id: row.get("target_field_id"),
            source_cardinality: row.get("source_cardinality"),
            target_cardinality: row.get("target_cardinality"),
        })
        .collect();

    diagram.relationships = Some(relationships);

    // Get dependencies
    let dep_rows = sqlx::query("SELECT * FROM db_dependencies WHERE diagram_id = $1")
        .bind(&id)
        .fetch_all(&pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: format!("Failed to fetch dependencies: {}", e),
                }),
            )
        })?;

    use crate::models::Dependency;
    let dependencies: Vec<Dependency> = dep_rows
        .into_iter()
        .map(|row| Dependency {
            id: row.get("id"),
            diagram_id: row.get("diagram_id"),
            schema: row.get("schema"),
            table_id: row.get("table_id"),
            dependent_schema: row.get("dependent_schema"),
            dependent_table_id: row.get("dependent_table_id"),
        })
        .collect();

    diagram.dependencies = Some(dependencies);

    // Get areas
    let area_rows = sqlx::query("SELECT * FROM areas WHERE diagram_id = $1")
        .bind(&id)
        .fetch_all(&pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: format!("Failed to fetch areas: {}", e),
                }),
            )
        })?;

    use crate::models::Area;
    let areas: Vec<Area> = area_rows
        .into_iter()
        .map(|row| Area {
            id: row.get("id"),
            diagram_id: row.get("diagram_id"),
            name: row.get("name"),
            x: row.get("x"),
            y: row.get("y"),
            width: row.get("width"),
            height: row.get("height"),
            color: row.get("color"),
        })
        .collect();

    diagram.areas = Some(areas);

    // Get custom types
    let ct_rows = sqlx::query("SELECT * FROM db_custom_types WHERE diagram_id = $1")
        .bind(&id)
        .fetch_all(&pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: format!("Failed to fetch custom types: {}", e),
                }),
            )
        })?;

    use crate::models::CustomType;
    let custom_types: Vec<CustomType> = ct_rows
        .into_iter()
        .map(|row| CustomType {
            id: row.get("id"),
            diagram_id: row.get("diagram_id"),
            schema: row.get("schema"),
            r#type: row.get("type"),
            kind: row.get("kind"),
            values: row.get("values"),
            fields: row.get("fields"),
        })
        .collect();

    diagram.custom_types = Some(custom_types);

    // Get notes
    let note_rows = sqlx::query("SELECT * FROM notes WHERE diagram_id = $1")
        .bind(&id)
        .fetch_all(&pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: format!("Failed to fetch notes: {}", e),
                }),
            )
        })?;

    use crate::models::Note;
    let notes: Vec<Note> = note_rows
        .into_iter()
        .map(|row| Note {
            id: row.get("id"),
            diagram_id: row.get("diagram_id"),
            content: row.get("content"),
            x: row.get("x"),
            y: row.get("y"),
            width: row.get("width"),
            height: row.get("height"),
            color: row.get("color"),
        })
        .collect();

    diagram.notes = Some(notes);

    Ok(Json(diagram))
}

pub async fn health() -> &'static str {
    "ok"
}

