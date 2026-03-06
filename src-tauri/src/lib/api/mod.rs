use crate::errors::Result;
use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde_json::json;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::lib::{
    db::Database,
    storage::StorageService,
    task::TaskQueue,
};

pub struct HttpServer {
    port: u16,
    db: Arc<Mutex<Database>>,
    storage: Arc<Mutex<StorageService>>,
    task_queue: Arc<Mutex<TaskQueue>>,
}

impl HttpServer {
    pub fn new(
        port: u16,
        db: Arc<Mutex<Database>>,
        storage: Arc<Mutex<StorageService>>,
        task_queue: Arc<Mutex<TaskQueue>>,
    ) -> Self {
        Self {
            port,
            db,
            storage,
            task_queue,
        }
    }

    pub async fn start(self) -> Result<()> {
        let app = self.create_router();

        let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", self.port)).await?;
        tracing::info!("HTTP server listening on port {}", self.port);

        axum::serve(listener, app).await?;
        Ok(())
    }

    fn create_router(self) -> Router {
        Router::new()
            .route("/health", get(health_check))
            .route("/api/projects", get(list_projects).post(create_project))
            .with_state(AppState {
                db: self.db,
                storage: self.storage,
                task_queue: self.task_queue,
            })
    }
}

#[derive(Clone)]
struct AppState {
    db: Arc<Mutex<Database>>,
    storage: Arc<Mutex<StorageService>>,
    task_queue: Arc<Mutex<TaskQueue>>,
}

async fn health_check() -> impl IntoResponse {
    Json(json!({
        "status": "ok",
        "service": "noidea"
    }))
}

async fn list_projects(State(state): State<AppState>) -> impl IntoResponse {
    let db = state.db.lock().await;
    let projects = sqlx::query_as::<_, (String,)>(
        "SELECT id FROM projects"
    )
    .fetch_all(db.pool())
    .await;

    match projects {
        Ok(rows) => {
            let project_ids: Vec<String> = rows.into_iter().map(|r| r.0).collect();
            Json(json!({ "projects": project_ids }))
        }
        Err(e) => {
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": e.to_string() })))
        }
    }
}

async fn create_project(
    State(state): State<AppState>,
    Json(payload): create_project::Payload,
) -> impl IntoResponse {
    let id = uuid::Uuid::new_v4().to_string();
    let now = chrono::Utc::now().timestamp();

    let db = state.db.lock().await;
    let result = sqlx::query(
        "INSERT INTO projects (id, name, description, created_at, updated_at, settings) VALUES (?1, ?2, ?3, ?4, ?5, ?6)"
    )
    .bind(&id)
    .bind(&payload.name)
    .bind(&payload.description)
    .bind(now)
    .bind(now)
    .bind(serde_json::to_string(&payload.settings).unwrap_or_default())
    .execute(db.pool())
    .await;

    match result {
        Ok(_) => {
            (StatusCode::CREATED, Json(json!({ "id": id })))
        }
        Err(e) => {
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": e.to_string() })))
        }
    }
}

mod create_project {
    use serde::Deserialize;

    #[derive(Deserialize)]
    pub struct Payload {
        pub name: String,
        pub description: Option<String>,
        pub settings: Option<serde_json::Value>,
    }
}
