use axum::Router;

pub mod projects;

pub fn create_routes() -> Router<crate::api::server::AppState> {
    Router::new()
        .nest("/projects", projects::router())
}
