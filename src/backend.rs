// use axum::{http::StatusCode, routing::get, Router};
//
// use crate::api::get_price_points_json;
//
// pub fn backend_router() -> Router {
//     let router = Router::new()
//         .route("/api/checkHealth", get(StatusCode::OK))
//         .route("/api/pricePoints", get(get_price_points_json));
//
//     return router;
// }
