use axum::{
    Router,
    routing::get,
    response::Json,
    http::StatusCode,
};
use tower::ServiceBuilder;
use tower_http::cors::{CorsLayer, Any};
use serde_json::json;
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::api::routes;
use crate::errors::Result;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: Arc<RwLock<Option<sqlx::SqlitePool>>>,
}

pub struct HttpServer {
    port: u16,
}

impl HttpServer {
    pub fn new(port: u16) -> Self {
        HttpServer { port }
    }

    pub async fn start(&self, db_pool: sqlx::SqlitePool) -> Result<()> {
        let state = AppState {
            db_pool: Arc::new(RwLock::new(Some(db_pool))),
        };

        let cors = CorsLayer::new()
            .allow_origin(Any)
            .allow_methods(Any)
            .allow_headers(Any);

        let app = Router::new()
            .route("/health", get(health_check))
            .route("/api/projects", get(routes::projects::get_projects))
            .nest("/api", routes::create_routes())
            .layer(ServiceBuilder::new().layer(cors))
            .with_state(state);

        let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", self.port)).await?;

        println!("HTTP server listening on http://0.0.0.0:{}", self.port);

        axum::serve(listener, app).await?;

        Ok(())
    }
}

async fn health_check() -> std::result::Result<Json<serde_json::Value>, StatusCode> {
    Ok(Json(json!({
        "status": "ok",
        "service": "popidea"
    })))
}
