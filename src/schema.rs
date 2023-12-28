diesel::table! {
    price_points (id) {
        id -> Int4,
        ticker_id -> Int4,
        date -> Timestamp,
        price -> Float8,
    }
}

diesel::table! {
    tickers (id) {
        id -> Int4,
        ticker_name -> Text,
        real_name -> Nullable<Text>,
        market_cap -> Float8,
    }
}

diesel::joinable!(price_points -> tickers (ticker_id));

diesel::allow_tables_to_appear_in_same_query!(price_points, tickers,);
