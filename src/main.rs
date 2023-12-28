use std::sync::Mutex;

mod database;
mod models;
mod routes;

#[allow(unused)]
use {
    axum::{
        http::StatusCode,
        routing::{get, post},
        Json, Router,
    },
    database::establish_conn,
    dotenvy::dotenv,
    models::*,
    routes::get_price_points_json,
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

    let mut conn = database::establish_conn().await;

    let app = Router::new()
        .route("/checkHealth", get(StatusCode::OK))
        .route("/getPricePointsJson", get(get_price_points_json))
        .layer(TraceLayer::new_for_http())
        .with_state(conn);

    println!("Listening on {}", &addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
