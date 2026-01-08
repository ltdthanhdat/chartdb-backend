use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Diagram {
    pub id: String,
    pub name: String,
    #[serde(rename = "databaseType")]
    pub database_type: String,
    #[serde(rename = "databaseEdition")]
    pub database_edition: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(rename = "updatedAt")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub tables: Option<Vec<Table>>,
    pub relationships: Option<Vec<Relationship>>,
    pub dependencies: Option<Vec<Dependency>>,
    pub areas: Option<Vec<Area>>,
    #[serde(rename = "customTypes")]
    pub custom_types: Option<Vec<CustomType>>,
    pub notes: Option<Vec<Note>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Table {
    pub id: String,
    #[serde(rename = "diagramId")]
    pub diagram_id: String,
    pub name: String,
    pub schema: Option<String>,
    pub x: Option<f64>,
    pub y: Option<f64>,
    pub width: Option<f64>,
    pub color: Option<String>,
    pub comment: Option<String>,
    #[serde(rename = "isView")]
    pub is_view: Option<bool>,
    #[serde(rename = "isMaterializedView")]
    pub is_materialized_view: Option<bool>,
    pub order: Option<i32>,
    pub fields: serde_json::Value,
    pub indexes: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Relationship {
    pub id: String,
    #[serde(rename = "diagramId")]
    pub diagram_id: String,
    pub name: Option<String>,
    #[serde(rename = "sourceSchema")]
    pub source_schema: Option<String>,
    #[serde(rename = "sourceTableId")]
    pub source_table_id: String,
    #[serde(rename = "targetSchema")]
    pub target_schema: Option<String>,
    #[serde(rename = "targetTableId")]
    pub target_table_id: String,
    #[serde(rename = "sourceFieldId")]
    pub source_field_id: Option<String>,
    #[serde(rename = "targetFieldId")]
    pub target_field_id: Option<String>,
    #[serde(rename = "sourceCardinality")]
    pub source_cardinality: Option<String>,
    #[serde(rename = "targetCardinality")]
    pub target_cardinality: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Dependency {
    pub id: String,
    #[serde(rename = "diagramId")]
    pub diagram_id: String,
    pub schema: Option<String>,
    #[serde(rename = "tableId")]
    pub table_id: String,
    #[serde(rename = "dependentSchema")]
    pub dependent_schema: Option<String>,
    #[serde(rename = "dependentTableId")]
    pub dependent_table_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Area {
    pub id: String,
    #[serde(rename = "diagramId")]
    pub diagram_id: String,
    pub name: Option<String>,
    pub x: Option<f64>,
    pub y: Option<f64>,
    pub width: Option<f64>,
    pub height: Option<f64>,
    pub color: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomType {
    pub id: String,
    #[serde(rename = "diagramId")]
    pub diagram_id: String,
    pub schema: Option<String>,
    #[serde(rename = "type")]
    pub r#type: String,
    pub kind: Option<String>,
    pub values: Option<serde_json::Value>,
    pub fields: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Note {
    pub id: String,
    #[serde(rename = "diagramId")]
    pub diagram_id: String,
    pub content: Option<String>,
    pub x: Option<f64>,
    pub y: Option<f64>,
    pub width: Option<f64>,
    pub height: Option<f64>,
    pub color: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct PushRequest {
    pub diagram: Diagram,
}

#[derive(Debug, Serialize)]
pub struct PushResponse {
    pub success: bool,
    #[serde(rename = "diagramId")]
    pub diagram_id: String,
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

