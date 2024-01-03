mod api;
mod backend;
mod database;
mod frontend;
mod models;

use {
    api::{get_price_points_json, index},
    axum::{
        http::StatusCode,
        routing::{get, post},
        Json, Router,
    },
    database::establish_conn,
    dotenvy::dotenv,
    models::*,
    serde::{Deserialize, Serialize},
    sqlx::PgPool,
    std::sync::Arc,
    tower_http::trace::TraceLayer,
};

#[tokio::main]
async fn main() {
    dotenv().ok();

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let port = std::env::var("PORT").unwrap_or("3000".to_string());
    let addr = format!("127.0.0.1:{}", port);

    let conn = database::establish_conn().await;

    let app = Router::new()
        .route("/", get(index))
        .route("/api/checkHealth", get(StatusCode::OK))
        .route("/api/pricePoints", get(get_price_points_json))
        //.merge(frontend::frontend_router())
        .layer(TraceLayer::new_for_http())
        .with_state(conn);

    println!("Listening on {}", &addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
