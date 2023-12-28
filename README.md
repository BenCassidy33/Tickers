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

Query Format

```javascript
{
  'ticker_name': 'TICK',
  'range': {
        'start': 'start_date',
        'end': 'end_date'
    }
}

```

When propted correctly, the server will respond with this format:

```javascript
// response from server:
{
  'ticker_name': 'TICK',
  'range': {
        'start': 'start_date',
        'end': 'end_date'
    },
    'prices': [
        {
            "id": 0, // price date determined by database
            "ticker_id": 1, // id of the refernced ticker symbol as determined by the database
            "date", "price_date", // the date of the closing price
            "price", 100.00 // price as a float
        },
        ...
    ]
}
```

Example Query:

```javascript
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

- HTTP 200: Success! The ticker is valid, and the date range is correctly set.
- HTTP 400: Bad Request. The JSON payload format is incorrect.
- HTTP 404: Not Found. The ticker does not exist or is invalid.
- HTTP 416: Range Not Satisfiable. The specified date range exceeds the available data.
- HTTP 422: Unprocessable Entity. The date range format is incorrect.

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
