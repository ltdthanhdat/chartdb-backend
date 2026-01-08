mod handlers;
mod models;
mod routes;

use anyhow::Result;
use sqlx::PgPool;
use std::env;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();

    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://chartdb:chartdb@localhost:5432/chartdb".to_string());

    let pool = PgPool::connect(&database_url).await?;

    // Run migrations
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .map_err(|e| anyhow::anyhow!("Migration failed: {}", e))?;

    let app = routes::create_router(pool).layer(CorsLayer::permissive());

    let port = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>()?;

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
    tracing::info!("Server running on port {}", port);

    axum::serve(listener, app).await?;

    Ok(())
}
