use axum::Router;
use crate::api::routes::projects;

pub mod projects;

pub fn create_routes() -> Router<crate::api::server::AppState> {
    Router::new()
        .nest("/projects", projects::router())
}
