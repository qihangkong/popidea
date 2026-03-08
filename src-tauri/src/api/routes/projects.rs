use axum::{
    Router,
    routing::get,
    extract::State,
    response::Json,
    http::StatusCode,
};
use serde_json::json;
use crate::api::server::AppState;
use crate::db::crud;
use crate::errors::Result;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(get_projects))
}

pub async fn get_projects(
    State(state): State<AppState>,
) -> std::result::Result<Json<serde_json::Value>, StatusCode> {
    let db_pool = state.db_pool.read().await;
    
    if let Some(pool) = db_pool.as_ref() {
        match crud::get_projects(pool).await {
            Ok(projects) => Ok(Json(json!({
                "success": true,
                "data": projects
            }))),
            Err(e) => {
                eprintln!("Error fetching projects: {}", e);
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    } else {
        Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}
