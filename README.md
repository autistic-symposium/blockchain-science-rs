# 🐊🤖 COINGATOR: a statistical rusty searcher 👾✨

<br>

<br>

<p align="center">
<img src="https://user-images.githubusercontent.com/1130416/210849647-bad9bc97-2193-4e9e-87c6-f75c8dd21712.jpeg" width="50%" align="center" style="padding:1px;border:1px solid black;"/>
 </p>

<br>


### tl; dr



##### 🐊 this program implements a bot running statistical strategies on several exchanges. It's called coingator because it's a cute animal that rhymes with cointegratoooor.

##### 📚 for more details about this project, check my mirror post: **[bot #3: coingator, a rusty cointegratooor searcher]()**.

##### 🚨 disclaimer: i open-source my projects because i believe in the oss ethos. you might or might not profit from it, but this is not my problem. in the mev world, nobody is going to (explicitly) handle you alphas. i am not responsible for anything you do with my free code.

<br>

---

### cointegration strategy

<br>

> *One of the most well-known strategies among different **algorithmic trading methods** is the **statistical arbitrage strategy**: a profitable situation stemming from **pricing inefficiencies among financial markets** - a mere strategy to obtain profit by applying **past statistics**.*

<br>


[Cointegration](https://en.wikipedia.org/wiki/Cointegration) is the **test correlation between two or more non-stationary time series** for a specified period (identifying long-run parameters and determining when stationary time series do not depart from equilibrium).

<br>

>  *Formally, if **(X,Y,Z)** are each integrated of order **d**, and there exist coefficients **a, b, c** such that **aX + bY + cZ** is integrated of the order less than **d**, then **X**, **Y**, and **Z** are cointegrated.*

<br>

In this package, we implement this strategy with the following steps:

```
    1. general websocket monitoring
    1. retrieving of a list of tradeable symbols
    2. generation of price history
    3. identification of cointegrated pairs
    5. backtesting and finding trading signals
    6. deployment of a custom bot
```

<br>


---
### main features of this package

<br>

#### monitoring

we use websockets and a CEX API of choice (*e.g.*, [bybit's API](https://www.bybit.com/en-US/)) to monitor public topics and info for:

```
    * crypto derivatives
    * pairs of crypto derivatives 
    * inverse perpetual contracts
    * spot pair orderbooks
```

<br>


#### trading bot


to run a trading bot, you must add the following info to the `.env` file: 

```
    * pair symbols
    * price range
    * number of grids
    * total investment
```


<br>

---

## local set up

<br>

1. make sure you have [rust](https://www.rust-lang.org/tools/install) installed.
2. get your API creds from the exchange you are using (e.g. [bybit]((https://testnet.bybit.com).
3. add info to a `.env` file:

```
cp .env.example .env
vim .env
```


then install **coingator** with:

```
make build
```

run with:

```
make run
```



<br>

---

## running coingator


<br>

<img width="500" alt="Screen Shot 2023-02-04 at 4 13 02 PM" src="https://user-images.githubusercontent.com/1130416/216794978-61ecae68-e834-4892-9512-c24b55359c38.png">


<br>

<br>

read on for more dets.


<br>

---

#### 1) subscribing to topics for a crypto derivative asset

<br>

> ***Crypto derivatives** are financial contracts that derive their values from underlying assets. **Crypto futures** contracts are proxy tools to speculate on the future prices of cryptocurrencies or to be used to hedge against their price changes.*

<br>

select `1` to open a websocket and subscribe to the following derivative's topics:

```
    - depths
    - trades
    - book tickers
    - realtimes
```



<br>

example output:


```
🐊 subscribing to websockets for: "ETHUSDT"
✅ depth: ResponseV2 { topic: "depth", params: ResCommonParamsV2 { binary: "false", symbol: "ETHUSDT", symbol_name: "ETHUSDT" }, data: Depth { t: 1672627313579, s: "ETHUSDT", v: "1672627313579_1", b: [OrderBookItem("1196.53", "6.81229"), OrderBookItem("1196.52", "10"), OrderBookItem("1196.51", "0.08357"), OrderBookItem("1196.46", "0.01876"), OrderBookItem("1196.44", "0.16778"), OrderBookItem("1196.4", "10.00414"), OrderBookItem("1196.39", "3.62"), OrderBookItem("1196.33", "11.98536"), OrderBookItem("1196.32", "4.37768"), OrderBookItem("1196.31", "0.25238"), OrderBookItem("1196.3", "2.44169"), OrderBookItem("1196.28", "6.48"), OrderBookItem("1196.27", "3.46"), OrderBookItem("1196.24", "0.001"), OrderBookItem("1196.22", "1.52901"), OrderBookItem("1196.21", "3.14679"), OrderBookItem("1196.19", "1.54244"), OrderBookItem("1196.18", "1.24823"), OrderBookItem("1196.16", "6.75"), OrderBookItem("1196.15", "4.35209"), OrderBookItem("1196.14", "0.30445"), OrderBookItem("1196.13", "0.41108"), OrderBookItem("1196.06", "16.713"), OrderBookItem("1196.04", "5"), OrderBookItem("1196.01", "9.5338"), OrderBookItem("1196", "4.009"), OrderBookItem("1195.99", "1.7134"), OrderBookItem("1195.97", "1.2251"), OrderBookItem("1195.95", "26.34158"), OrderBookItem("1195.91", "5"), OrderBookItem("1195.9", "0.08359"), OrderBookItem("1195.85", "0.89955"), OrderBookItem("1195.83", "0.16989"), OrderBookItem("1195.82", "1.25766"), OrderBookItem("1195.8", "2.515"), OrderBookItem("1195.76", "3.57"), OrderBookItem("1195.75", "5"), OrderBookItem("1195.72", "0.1672"), OrderBookItem("1195.7", "3.78046"), OrderBookItem("1195.69", "1.41835")], a: [OrderBookItem("1196.69", "2.02591"), OrderBookItem("1196.72", "0.001"), OrderBookItem("1196.8", "0.0192"), OrderBookItem("1196.84", "0.25238"), OrderBookItem("1196.9", "3"), OrderBookItem("1196.92", "1.94"), OrderBookItem("1196.93", "0.16315"), OrderBookItem("1196.94", "0.89955"), OrderBookItem("1196.95", "2.02888"), OrderBookItem("1196.96", "4.00607"), OrderBookItem("1196.98", "3.687"), OrderBookItem("1197", "0.01624"), OrderBookItem("1197.02", "3.05537"), OrderBookItem("1197.05", "0.84081"), OrderBookItem("1197.08", "1.51308"), OrderBookItem("1197.09", "3.174"), OrderBookItem("1197.1", "0.03835"), OrderBookItem("1197.13", "1.5768"), OrderBookItem("1197.15", "24.16"), OrderBookItem("1197.18", "0.83532"), OrderBookItem("1197.2", "2.18155"), OrderBookItem("1197.22", "1.52477"), OrderBookItem("1197.23", "1.71323"), OrderBookItem("1197.27", "9.5338"), OrderBookItem("1197.29", "2.73673"), OrderBookItem("1197.32", "0.0503"), OrderBookItem("1197.35", "25.07989"), OrderBookItem("1197.42", "16.713"), OrderBookItem("1197.43", "0.03916"), OrderBookItem("1197.44", "0.83499"), OrderBookItem("1197.5", "46.17"), OrderBookItem("1197.52", "0.12796"), OrderBookItem("1197.53", "1.15349"), OrderBookItem("1197.55", "25.75042"), OrderBookItem("1197.6", "0.09146"), OrderBookItem("1197.65", "0.26999"), OrderBookItem("1197.66", "0.89955"), OrderBookItem("1197.67", "12.01463"), OrderBookItem("1197.71", "0.8725"), OrderBookItem("1197.72", "0.1")] } }
✅ trade: ResponseV2 { topic: "trade", params: ResCommonParamsV2 { binary: "false", symbol: "ETHUSDT", symbol_name: "ETHUSDT" }, data: Trade { v: "2280000000030465445", t: 1672627309524, p: "1196.69", q: "0.00083", m: true } }
✅ book ticker: ResponseV2 { topic: "bookTicker", params: ResCommonParamsV2 { binary: "false", symbol: "ETHUSDT", symbol_name: "ETHUSDT" }, data: BookTicker { symbol: "ETHUSDT", bid_price: "1196.53", bid_qty: "6.81229", ask_price: "1196.69", ask_qty: "2.02591", time: 1672627312138 } }
✅ realtimes: ResponseV2 { topic: "realtimes", params: ResCommonParamsV2 { binary: "false", symbol: "ETHUSDT", symbol_name: "ETHUSDT" }, data: Realtimes { t: 1672627309524, s: "ETHUSDT", c: "1196.69", h: "1204.66", l: "1190.96", o: "1195.91", v: "20134.37723", qv: "24113384.0565187", m: "0.0007" } }
✅ pong: Pong { pong: 1672627314329 }
(...)
```

<br>

---

#### 2) subscribing to topics for a pair of crypto derivative assets

select `2` to open a websocket and subscribe the pair to the following topics:


```
    - orderbook l2 25, 200
    - trades
    - instruments info
    - k-lines
    - liquidations
```


<br>

example output:

```
🐊 subscribing to websockets for: ["ETHUSDT", "BTCUSDT"]
✅ instrument info snapshot: Response { topic: "instrument_info.100ms.ETHUSDT", res_type: "snapshot", data: InstrumentInfoSnapshot { id: 2, symbol: "ETHUSDT", last_price_e4: "11966500", last_price: "1196.65", bid1_price_e4: "11966000", bid1_price: "1196.60", ask1_price_e4: "11966500", ask1_price: "1196.65", last_tick_direction: "ZeroPlusTick", prev_price_24h_e4: "11939000", prev_price_24h: "1193.90", price_24h_pcnt_e6: "2303", high_price_24h_e4: "12050500", high_price_24h: "1205.05", low_price_24h_e4: "11913500", low_price_24h: "1191.35", prev_price_1h_e4: "11962500", prev_price_1h: "1196.25", price_1h_pcnt_e6: "334", mark_price_e4: "11966100", mark_price: "1196.61", index_price_e4: "11965400", index_price: "1196.54", open_interest_e8: "58516141000000", total_turnover_e8: "4093197159274450000", turnover_24h_e8: "35260448999150020", total_volume_e8: "38356353516000000", volume_24h_e8: "29427055999999", funding_rate_e6: "100", predicted_funding_rate_e6: "100", cross_seq: "24972973797", created_at: "2022-03-31T03:56:16.000Z", updated_at: "2023-01-02T03:14:29.000Z", next_funding_time: "2023-01-02T08:00:00Z", count_down_hour: "5", funding_rate_interval: "8", settle_time_e9: "0", delisting_status: "0" }, cross_seq: "24972976935", timestamp_e6: "1672629272903187" }
(...)
```



<br>

---

#### 3) subscribing to inverse perpetuals info

<br>


> *A **perpetual contract** has no expiration date. At Bybit, funding occurs every amount of hours, and they use the interest rate and the premium index to calculate the funding through **Time-Weighted-Average-Price (TWAP)** over the series of minutes rates.*

<br>

select `3` to open a websocket and subscribe to the following topics:



```
    - orderbook l2 25, 200
    - trades
    - insurances
    - instrument info
    - k-lines
    - liquidations
```


<br>

example output:

```

```



<br>


---



#### 4) subscribing to spot local orderbook

<br>

> *In the **spot market**, one can buy and sell cryptocurrencies for **immediate delivery**. They are directly transferred between market participants (buyers and sellers), which have **direct ownership of the assets** and are entitled to **economic benefits**, such as voting or staking participation.*

<br>

select `4` to open a websocket and subscribe to spot info on the following topics:


```
    - trades
    - diff depths
```


<br>

example output:



```
✨🐊 ETHUSDT order book

💰 price              🛍 quantity            
1217.31              1.71858             
1217.29              0.9398              
1217.28              3.17709             
1217.25              1.71                
1217.24              3.27                
1217.23              5                   
1217.19              0.25474             
1217.15              0.08218             
1217.14              5.18416             
1217.09              5.0018              

🔻 1216.27

1216.98              0.1489              
1216.94              0.08218             
1216.92              0.3                 
1216.85              6.01827             
1216.8               0.09138             
1216.71              0.3                 
1216.7               2.06                
1216.67              0.08347             
1216.66              5.03956             
1216.63              3.31   
```



<br>

---

#### 5. get cointegration for a given pair 



select `5` to find cointegration for a given pair.



<br>


---

#### 6. deploying customized coingator bot



select `6` to run a customized bot using this strategy.




<br>

---


## resources

<br>

* [Cointbot, my cointegration bots in Python, with full visual analysis of cointegration pairs](https://github.com/go-outside-labs/blockchain-science-py/tree/main/cointegration-bots).


<br>

* [Cointegration-Based Pairs Trading Strategy in the Cryptocurrency Market (arxiv:2109.10662)](https://arxiv.org/abs/2109.10662)
    - *"By considering the main limitations in the market microstructure, our
strategy exceeds the naive buy-and-hold approach in the Bitmex exchange. Another
significant finding is that we implement a numerous collection of cryptocurrency coins
to formulate the model’s spread, which improves the risk-adjusted profitability of the
pairs trading strategy. Besides, the strategy’s maximum drawdown level is reasonably
low, which makes it useful to be deployed. The results also indicate that a class of
coins has better potential arbitrage opportunities than others."*

<br>

* [Constructing Cointegrated Cryptocurrency Portfolios](https://towardsdatascience.com/constructing-cointegrated-cryptocurrency-portfolios-d0a27922891e)
    - *As the cryptocurrency market continues to grow with new coins and new exchanges, it’s very important for individual investors, crypto-fund managers, as well as regulators to understand the price dependency among all cryptocurrencies, along with their derivatives.*
