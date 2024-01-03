use axum::{handler::HandlerWithoutStateExt, http::StatusCode, Router};
use tower_http::services::ServeDir;

pub fn frontend_router() -> Router {
    let router = Router::new()
        .fallback_service(ServeDir::new("frontend").not_found_service(handle_error.into_service()));

    return router;
}

pub async fn handle_error() -> (StatusCode, &'static str) {
    (StatusCode::NOT_FOUND, "Files not found")
}
