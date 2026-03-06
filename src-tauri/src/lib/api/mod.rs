use crate::errors::Result;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post, put},
    Json, Router,
};
use serde_json::json;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::lib::{
    db::{crud, Database},
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
            .route("/api/projects/:id", get(get_project).put(update_project).delete(delete_project))
            .route("/api/projects/:id/episodes", get(list_episodes).post(create_episode))
            .route("/api/episodes/:id", get(get_episode).put(update_episode).delete(delete_episode))
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
    match crud::ProjectRepository::list(db.pool()).await {
        Ok(projects) => {
            (StatusCode::OK, Json(json!({ "projects": projects })))
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
    let db = state.db.lock().await;
    match crud::ProjectRepository::create(db.pool(), payload.name, payload.description).await {
        Ok(project) => {
            (StatusCode::CREATED, Json(json!({ "project": project })))
        }
        Err(e) => {
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": e.to_string() })))
        }
    }
}

async fn get_project(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let db = state.db.lock().await;
    match crud::ProjectRepository::get_by_id(db.pool(), &id).await {
        Ok(project) => {
            (StatusCode::OK, Json(json!({ "project": project })))
        }
        Err(e) => {
            (StatusCode::NOT_FOUND, Json(json!({ "error": e.to_string() })))
        }
    }
}

async fn update_project(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(payload): update_project::Payload,
) -> impl IntoResponse {
    let db = state.db.lock().await;
    match crud::ProjectRepository::update(db.pool(), &id, payload.name, payload.description).await {
        Ok(project) => {
            (StatusCode::OK, Json(json!({ "project": project })))
        }
        Err(e) => {
            (StatusCode::NOT_FOUND, Json(json!({ "error": e.to_string() })))
        }
    }
}

async fn delete_project(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let db = state.db.lock().await;
    match crud::ProjectRepository::delete(db.pool(), &id).await {
        Ok(_) => {
            (StatusCode::NO_CONTENT, Json(json!({})))
        }
        Err(e) => {
            (StatusCode::NOT_FOUND, Json(json!({ "error": e.to_string() })))
        }
    }
}

async fn list_episodes(
    State(state): State<AppState>,
    Path(project_id): Path<String>,
) -> impl IntoResponse {
    let db = state.db.lock().await;
    match crud::EpisodeRepository::list_by_project(db.pool(), &project_id).await {
        Ok(episodes) => {
            (StatusCode::OK, Json(json!({ "episodes": episodes })))
        }
        Err(e) => {
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": e.to_string() })))
        }
    }
}

async fn create_episode(
    State(state): State<AppState>,
    Path(project_id): Path<String>,
    Json(payload): create_episode::Payload,
) -> impl IntoResponse {
    let db = state.db.lock().await;
    match crud::EpisodeRepository::create(db.pool(), project_id, payload.name, payload.content).await {
        Ok(episode) => {
            (StatusCode::CREATED, Json(json!({ "episode": episode })))
        }
        Err(e) => {
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": e.to_string() })))
        }
    }
}

async fn get_episode(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let db = state.db.lock().await;
    match crud::EpisodeRepository::get_by_id(db.pool(), &id).await {
        Ok(episode) => {
            (StatusCode::OK, Json(json!({ "episode": episode })))
        }
        Err(e) => {
            (StatusCode::NOT_FOUND, Json(json!({ "error": e.to_string() })))
        }
    }
}

async fn update_episode(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(payload): update_episode::Payload,
) -> impl IntoResponse {
    let db = state.db.lock().await;
    match crud::EpisodeRepository::update(db.pool(), &id, payload.name, payload.content).await {
        Ok(episode) => {
            (StatusCode::OK, Json(json!({ "episode": episode })))
        }
        Err(e) => {
            (StatusCode::NOT_FOUND, Json(json!({ "error": e.to_string() })))
        }
    }
}

async fn delete_episode(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let db = state.db.lock().await;
    match crud::EpisodeRepository::delete(db.pool(), &id).await {
        Ok(_) => {
            (StatusCode::NO_CONTENT, Json(json!({})))
        }
        Err(e) => {
            (StatusCode::NOT_FOUND, Json(json!({ "error": e.to_string() })))
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

mod update_project {
    use serde::Deserialize;

    #[derive(Deserialize)]
    pub struct Payload {
        pub name: Option<String>,
        pub description: Option<String>,
    }
}

mod create_episode {
    use serde::Deserialize;

    #[derive(Deserialize)]
    pub struct Payload {
        pub name: String,
        pub content: Option<String>,
    }
}

mod update_episode {
    use serde::Deserialize;

    #[derive(Deserialize)]
    pub struct Payload {
        pub name: Option<String>,
        pub content: Option<String>,
    }
}
