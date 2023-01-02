# 🐊🤖 COINGATOR: Statistical Arb Seacher 👾✨

<br>

**This program implements an arb seacher running statistical arbitrage strategies, such as cointegration**

For more details about this searcher on my Mirror post, **[bot #2: coingator, a rusty statistical arb searcher]()**.


<br>

---

## strategies

#### statistical methods

* [cointegration](https://en.wikipedia.org/wiki/Cointegration): test correlation between two or more non-stationary time series in the long run or for a specified period (identifying long run parameters and determining when stationary time series do not depart from equilibrium.)

#### cexes

* [bybit](https://www.bybit.com/en-US/)
* [binance](https://www.binance.us/) (tba)
* [bitmex](https://www.bitmex.com/) (tba)

#### dexes


<br>

---

## setting up

Make sure you have [rust](https://www.rust-lang.org/tools/install) installed.

Add `.env` info:

```
cp .env.example .env
vim .env
```


Install with:

```
make build
```

Run with:

```
make run
```



<br>

---

## running with bybit

#### subscribing for topics for a coin

Select `coin`:

```
🏭 welcome to coingator 🪙. type your option:

➡ coin: subscribe to all topics for a coin (eg. ETHUSDT)
➡ pairs: subscribe to order books topics for a pair (e.g. BTCUSDT, ETHUSDT)

coin
```

<br>

Example output:


```
👾 subcribing to websockets for: "ETHUSDT"
✅ depth: ResponseV2 { topic: "depth", params: ResCommonParamsV2 { binary: "false", symbol: "ETHUSDT", symbol_name: "ETHUSDT" }, data: Depth { t: 1672627313579, s: "ETHUSDT", v: "1672627313579_1", b: [OrderBookItem("1196.53", "6.81229"), OrderBookItem("1196.52", "10"), OrderBookItem("1196.51", "0.08357"), OrderBookItem("1196.46", "0.01876"), OrderBookItem("1196.44", "0.16778"), OrderBookItem("1196.4", "10.00414"), OrderBookItem("1196.39", "3.62"), OrderBookItem("1196.33", "11.98536"), OrderBookItem("1196.32", "4.37768"), OrderBookItem("1196.31", "0.25238"), OrderBookItem("1196.3", "2.44169"), OrderBookItem("1196.28", "6.48"), OrderBookItem("1196.27", "3.46"), OrderBookItem("1196.24", "0.001"), OrderBookItem("1196.22", "1.52901"), OrderBookItem("1196.21", "3.14679"), OrderBookItem("1196.19", "1.54244"), OrderBookItem("1196.18", "1.24823"), OrderBookItem("1196.16", "6.75"), OrderBookItem("1196.15", "4.35209"), OrderBookItem("1196.14", "0.30445"), OrderBookItem("1196.13", "0.41108"), OrderBookItem("1196.06", "16.713"), OrderBookItem("1196.04", "5"), OrderBookItem("1196.01", "9.5338"), OrderBookItem("1196", "4.009"), OrderBookItem("1195.99", "1.7134"), OrderBookItem("1195.97", "1.2251"), OrderBookItem("1195.95", "26.34158"), OrderBookItem("1195.91", "5"), OrderBookItem("1195.9", "0.08359"), OrderBookItem("1195.85", "0.89955"), OrderBookItem("1195.83", "0.16989"), OrderBookItem("1195.82", "1.25766"), OrderBookItem("1195.8", "2.515"), OrderBookItem("1195.76", "3.57"), OrderBookItem("1195.75", "5"), OrderBookItem("1195.72", "0.1672"), OrderBookItem("1195.7", "3.78046"), OrderBookItem("1195.69", "1.41835")], a: [OrderBookItem("1196.69", "2.02591"), OrderBookItem("1196.72", "0.001"), OrderBookItem("1196.8", "0.0192"), OrderBookItem("1196.84", "0.25238"), OrderBookItem("1196.9", "3"), OrderBookItem("1196.92", "1.94"), OrderBookItem("1196.93", "0.16315"), OrderBookItem("1196.94", "0.89955"), OrderBookItem("1196.95", "2.02888"), OrderBookItem("1196.96", "4.00607"), OrderBookItem("1196.98", "3.687"), OrderBookItem("1197", "0.01624"), OrderBookItem("1197.02", "3.05537"), OrderBookItem("1197.05", "0.84081"), OrderBookItem("1197.08", "1.51308"), OrderBookItem("1197.09", "3.174"), OrderBookItem("1197.1", "0.03835"), OrderBookItem("1197.13", "1.5768"), OrderBookItem("1197.15", "24.16"), OrderBookItem("1197.18", "0.83532"), OrderBookItem("1197.2", "2.18155"), OrderBookItem("1197.22", "1.52477"), OrderBookItem("1197.23", "1.71323"), OrderBookItem("1197.27", "9.5338"), OrderBookItem("1197.29", "2.73673"), OrderBookItem("1197.32", "0.0503"), OrderBookItem("1197.35", "25.07989"), OrderBookItem("1197.42", "16.713"), OrderBookItem("1197.43", "0.03916"), OrderBookItem("1197.44", "0.83499"), OrderBookItem("1197.5", "46.17"), OrderBookItem("1197.52", "0.12796"), OrderBookItem("1197.53", "1.15349"), OrderBookItem("1197.55", "25.75042"), OrderBookItem("1197.6", "0.09146"), OrderBookItem("1197.65", "0.26999"), OrderBookItem("1197.66", "0.89955"), OrderBookItem("1197.67", "12.01463"), OrderBookItem("1197.71", "0.8725"), OrderBookItem("1197.72", "0.1")] } }
✅ trade: ResponseV2 { topic: "trade", params: ResCommonParamsV2 { binary: "false", symbol: "ETHUSDT", symbol_name: "ETHUSDT" }, data: Trade { v: "2280000000030465445", t: 1672627309524, p: "1196.69", q: "0.00083", m: true } }
✅ book ticker: ResponseV2 { topic: "bookTicker", params: ResCommonParamsV2 { binary: "false", symbol: "ETHUSDT", symbol_name: "ETHUSDT" }, data: BookTicker { symbol: "ETHUSDT", bid_price: "1196.53", bid_qty: "6.81229", ask_price: "1196.69", ask_qty: "2.02591", time: 1672627312138 } }
✅ realtimes: ResponseV2 { topic: "realtimes", params: ResCommonParamsV2 { binary: "false", symbol: "ETHUSDT", symbol_name: "ETHUSDT" }, data: Realtimes { t: 1672627309524, s: "ETHUSDT", c: "1196.69", h: "1204.66", l: "1190.96", o: "1195.91", v: "20134.37723", qv: "24113384.0565187", m: "0.0007" } }
✅ pong: Pong { pong: 1672627314329 }
```

<br>

#### subscribing for topics for a pair

Select `pairs`:

```
🏭 welcome to coingator 🪙. type your option:

➡ coin: subscribe to all topics for a coin (eg. ETHUSDT)
➡ pairs: subscribe to order books topics for a pair (e.g. BTCUSDT, ETHUSDT)

pairs
```

<br>

Example output:

```

```



<br>


---

## resources