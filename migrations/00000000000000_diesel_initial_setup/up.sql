CREATE TABLE IF NOT EXISTS tickers (
    id SERIAL PRIMARY KEY,
    ticker_name TEXT NOT NULL,
    real_name TEXT,
    market_cap FLOAT NOT NULL
);


CREATE TABLE IF NOT EXISTS price_points (
    id SERIAL PRIMARY KEY,
    ticker_id INT REFERENCES tickers(id) NOT NULL,
    date TIMESTAMP NOT NULL,
    price FLOAT NOT NULL
);
