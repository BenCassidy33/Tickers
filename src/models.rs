use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

pub mod globals {
    use chrono::NaiveDateTime;
    use lazy_static::lazy_static;
    pub static TIME_FORMAT: &'static str = "%Y-%m-%dT%H:%M:%S";

    lazy_static! {
        pub static ref START_DATE: NaiveDateTime =
            NaiveDateTime::parse_from_str("2023-06-01T04:00:00", TIME_FORMAT).unwrap();
        pub static ref END_DATE: NaiveDateTime =
            NaiveDateTime::parse_from_str("2023-12-01T04:00:00", TIME_FORMAT).unwrap();
    }
}

#[allow(unused)]
pub enum DateValidation {
    Valid,
    InvalidFormatting,
    InvalidRange,
    InvalidDateSequence,
}

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
pub struct ResultType {
    pub ticker_name: String,
    pub range: Option<DateRange>,
    pub prices: Vec<PricePoint>,
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub struct DateRange {
    pub start: NaiveDateTime,
    pub end: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestType {
    pub ticker_name: String,
    pub range: Option<DateRange>,
}
