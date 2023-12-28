mod database;
mod models;
mod schema;

#[allow(unused)]
use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use database::establish_conn;
use diesel::prelude::*;
use models::*;
#[allow(unused)]
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    use self::schema::tickers::dsl::*;

    let conn = &mut establish_conn();
    let res = tickers
        .filter(ticker_name.eq("nvda"))
        .limit(5)
        .select(Ticker::as_select())
        .load(conn)
        .expect("coult not load ticker");

    println!("Found: {}", res.len());

    // let app = Router::new().route("/checkHealth", get(StatusCode::OK));
    //
    // let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    // axum::serve(listener, app).await.unwrap();
}
