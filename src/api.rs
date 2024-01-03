use axum::response::Html;
use tracing::debug;

use {
    crate::models::{DateRange, PricePoint, RequestType, ResultType, TickerId},
    axum::http,
    axum::{extract::State, Json},
    chrono::{NaiveDateTime, Utc},
    sqlx::Row,
    sqlx::{postgres::PgPoolOptions, PgPool},
    std::sync::Arc,
};

pub async fn index() -> Html<String> {
    let data = std::fs::read_to_string("views/index.html").unwrap();
    Html(data)
}

pub async fn get_price_points_json(
    State(conn): State<PgPool>,
    Json(payload): Json<RequestType>,
) -> Result<(http::StatusCode, Json<ResultType>), http::StatusCode> {
    let ticker_id = sqlx::query_as!(
        TickerId,
        "SELECT id FROM tickers WHERE ticker_name = $1",
        payload.ticker_name
    )
    .fetch_optional(&conn)
    .await
    .map_err(|_| http::StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(http::StatusCode::NOT_FOUND)?;

    debug!("TOKEN ID: {}", ticker_id.id);

    if payload.range.is_some() {
        let start = payload.range.unwrap().start;
        let end = payload.range.unwrap().end;

        if start > end {
            return Err(http::StatusCode::RANGE_NOT_SATISFIABLE);
        }

        let price_points = sqlx::query_as!(
            PricePoint,
            "SELECT * FROM price_points WHERE ticker_id = $1 and date >= $2 and date <= $3",
            ticker_id.id,
            payload.range.unwrap().start,
            payload.range.unwrap().end,
        )
        .fetch_all(&conn)
        .await
        .map_err(|_| http::StatusCode::INTERNAL_SERVER_ERROR)?;

        if price_points.len() == 0 {
            return Err(http::StatusCode::NOT_FOUND);
        } else {
            return Ok((
                http::StatusCode::OK,
                Json(ResultType {
                    ticker_name: payload.ticker_name,
                    range: payload.range,
                    prices: price_points,
                }),
            ));
        }
    } else {
        let price_points = sqlx::query_as!(
            PricePoint,
            "SELECT * FROM price_points WHERE ticker_id = $1",
            ticker_id.id,
        )
        .fetch_all(&conn)
        .await
        .map_err(|_| http::StatusCode::INTERNAL_SERVER_ERROR)?;

        if price_points.len() == 0 {
            return Err(http::StatusCode::NOT_FOUND);
        } else {
            return Ok((
                http::StatusCode::OK,
                Json(ResultType {
                    ticker_name: payload.ticker_name,
                    range: None,
                    prices: price_points,
                }),
            ));
        }
    }
}
