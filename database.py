from pykrx import stock
from pykrx import bond
import time

dl = stock.get_market_ticker_list(market="ALL")
#df = stock.get_market_ohlcv("20230511", market="ALL")

for ticker in dl:
    df = stock.get_market_ohlcv("20220501", "20230511", ticker)
    df.to_csv(f"./data/{ticker}.csv", sep=' ')
    time.sleep(3)
