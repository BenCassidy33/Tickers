''' Schema

tickers (
    id SERIAL PRIMARY KEY,
    ticker_name TEXT NOT NULL,
    real_name TEXT,
    market_cap FLOAT NOT NULL
);

price_points (
    id SERIAL PRIMARY KEY,
    ticker_id INT REFERENCES tickers(id) NOT NULL,
    date TIMESTAMP NOT NULL,
    price FLOAT NOT NULL
);

'''

import psycopg2
import os
import json
from dotenv import load_dotenv
from urllib.parse import urlparse

env = load_dotenv(dotenv_path=os.path.abspath('../.env'))
url = os.getenv("DATABASE_URL")
print(url)
url = urlparse(url)

username = url.username
password = url.password
database = url.path[1:]
hostname = url.hostname
port = url.port

conn = psycopg2.connect(
    database = database,
    user = username,
    password = password,
    host = hostname,
    port = port
)

cur = conn.cursor()

with open("data/data.json", "r") as data:
    data = json.load(data)
    for ticker in data:
        ticker_name = ticker['symbol']
        cur.execute(f"SELECT id FROM tickers WHERE ticker_name = '{ticker_name}'")
        ticker_id = cur.fetchall()[0][0]
        date = ticker['from']
        price = ticker['close']
        print(f"Inserting into {ticker_name}, price: {price}, at: {date}")

        cur.execute(f"INSERT INTO price_points (ticker_id, date, price) VALUES ({ticker_id}, '{date} 04:00:00'::timestamp, {price})")
        conn.commit()


conn.close()
