import requests
import os
import time 
from datetime import datetime, timedelta
from dotenv import load_dotenv


stock_tickers = [
    "AAPL", 
    "MSFT", 
    "GOOGL",
    "AMZN", 
    "FB",   
    "TSLA", 
    "JPM",  
    "V",    
    "PYPL", 
    "NFLX", 
    "GS",   
    "DIS",  
    "IBM",  
    "BA",   
    "GE"    
]

env = load_dotenv(dotenv_path=os.path.abspath('../.env'))
api_key = os.getenv("STOCKS_API_KEY")

f = open("data/data.json", "w")
f.write('[')

def gen_date_time_range(start_date, end_date):
    start = datetime.strptime(start_date, "%Y-%m-%d")
    end = datetime.strptime(end_date, "%Y-%m-%d")

    curr = start
    dates = []

    while curr <= end:
        dates.append(curr.strftime("%Y-%m-%d"))
        curr += timedelta(days=1)

    return dates


dates = gen_date_time_range("2023-06-01", "2023-12-01")
print(len(dates))
max_len = len("Wait: 99")

def get_data():
    loop = 0
    round = 1
    print(f"Starting Round {round}")
    round += 1
    for tick in stock_tickers:
        for date in dates:
            url = f'https://api.polygon.io/v1/open-close/{tick}/{date}?adjusted=true&apiKey={api_key}'
            r = requests.get(url).json()
            print(r)

            if r['status'] == 'OK':
                f.write(str(r) + ',')
            loop += 1

            if loop == 5: 
                count = 60
                while count > 0:
                    print(f"Wait: {count}".ljust(max_len), end='\r')
                    count -= 1
                    time.sleep(1)

                loop = 0
                print(f"Starting Round {round}")
                round += 1


f.write(']')
f.close()

print(f"Time taken: {tme}")

