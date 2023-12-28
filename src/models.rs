use std::time::SystemTime;

use crate::schema::{price_points, tickers};
use diesel::prelude::*;

#[derive(Queryable, Selectable, Identifiable, Debug, PartialEq)]
#[diesel(table_name = tickers)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Ticker {
    pub id: i32,
    pub ticker_name: String,
    pub real_name: Option<String>,
    pub market_cap: f64,
}

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, PartialEq)]
#[diesel(belongs_to(Ticker))]
#[diesel(table_name = price_points)]
pub struct PricePoint {
    pub id: i32,
    pub ticker_id: i32,
    pub date: SystemTime,
    pub price: f64,
}

#[derive(Insertable)]
#[diesel(table_name = tickers)]
pub struct NewTicker {
    pub ticker_name: String,
    pub real_name: Option<String>,
    pub market_cap: f64,
}

#[derive(Insertable)]
#[diesel(table_name = price_points)]
pub struct NewPricePoint {
    pub ticker_id: i32,
    date: SystemTime,
    price: f64,
}
