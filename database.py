from pykrx import stock
from pykrx import bond
import time

dl = stock.get_market_ticker_list(market="ALL")
filePath = './data/list.csv'
with open(filePath, 'w') as lf:
    lf.write('\n'.join(dl))

for ticker in dl:
    df = stock.get_market_ohlcv("20210512", "20230512", ticker)
    print(df.columns)
    df.columns = ['open_price', 'high_price', 'low_price', 'close_price', 'volume', 'amount', 'performance']
    df.index.names = ['date']
    df.to_csv(f"./data/{ticker}.csv", mode='a',  sep=' ')
    time.sleep(3)
