# Tickers - Stock Ticker App

## Introduction

Welcome to Tickers, my personal project designed to past stock market data. This application, created using Rust and Python, showcases my skills in efficient data processing and software development.

## Features

- Stock data from June 1st 2023 - December 1st 2023
- Intuitive and user-friendly interface
- High-performance data processing using Rust

## Technologies

Rust:

- Axum, for api routing and http requests
- Askama, for html templating
- Chrono, for time types
- Sqlx, for database interaction,
- Tokio, for an asynchronous runtime
- Tower and Tracing, for tracing http requests

Python:

- Psycopg2, for database interaction
- Dotenv, for .env parsing,
- Urllib, for parsing Postgres Database Url

Others:

- Postgres, for data storage,
- Polygon.io, for stock data,
- Postman, for api testing,
- Sql for database initalization / migration as well as testing

## How It Works

The data was fetched and scraped from polygon.io using python and uploaded to a locally hosted postgres database. The program takies in a json query to /getPricePointsJson

```javascript
// example query:

const input = {
  ticker_name: 'AAPL', // set the ticker name to AAPL or Apple
};

// or get all dates within a range
const input = {
  ticker_name: 'AAPL',
  range: {
    // sets the range of price points from June 1st to December 1st
    start: '2023-06-09T4:00:00', // June 6th, 2023 at Closing
    end: '2023-11-11T4:00:00', // December 6th, 2023 at Closing
  },
};

// get the data from the server
const res = fetch(`http://${url}/getPricePointsJson`, {
  method: 'POST',
  headers: {
    'Content-Type': 'application/json',
  },
  body: JSON.stringify(input),
})
  .then((res) => res.json())
  .then((data) => {
    console.log(res);
  });

res = {
    "ticker_name": "AAPL",
    "range": {
        start: '2023-06-09T4:00:00',
        end: '2023-11-11T4:00:00',
    },

    "prices": [
        {
            "id": 16,
            "ticker_id": 2,
            "date": "2023-06-09T04:00:00",
            "price": 180.96
        },
        {
            "id": 17,
            "ticker_id": 2,
            "date": "2023-06-12T04:00:00",
            "price": 183.79
        },
        ...
    ]
}
```

## Responses

- Service will respond with a HTTP status code 200 if the ticker is found and the range is set correctly
- Service will respond with a HTTP status code 404 if the ticker is not valid/not found,
- Service will respond with a HTTP status code 416 if the range is outside the bound of stored dates
- Service will respond with a HTTP status code 422 if the range input is incorrectly formatted

## Constraints

- Tickers are limited to 14 different symbols and include:

  - "AAPL",
  - "MSFT",
  - "GOOGL",
  - "AMZN",
  - "TSLA",
  - "JPM",
  - "V",
  - "PYPL",
  - "NFLX",
  - "GS",
  - "DIS",
  - "IBM",
  - "BA",
  - "GE"

- Dates and prices are constrined from June 1st 2023 to December 1st 2023
- Some dates may be missing
- Range queries must follow the ISO 8601 time format (eg. "YYYY-MM-DDTHH:MM:SS" / "%Y-%m-%dT%H:%M:%S")
