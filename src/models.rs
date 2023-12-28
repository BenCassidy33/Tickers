use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(FromRow, Debug, Serialize, Deserialize)]
pub struct Ticker {
    pub id: i32,
    pub ticker_name: String,
    pub real_name: Option<String>,
    pub market_cap: f64,
}

#[derive(FromRow, Debug)]
pub struct TickerId {
    pub id: i32,
}

#[derive(FromRow, Debug, Serialize, Deserialize)]
pub struct PricePoint {
    pub id: i32,
    pub ticker_id: i32,
    pub date: NaiveDateTime,
    pub price: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestType {
    pub ticker_name: String,
    pub range: Option<DateRange>,
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub struct DateRange {
    pub start: NaiveDateTime,
    pub end: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResultType {
    pub ticker_name: String,
    pub range: DateRange,
    pub prices: Vec<PricePoint>,
}
