# Alpha Vantage vs Twelve Data — Common Endpoints

Both [Alpha Vantage](https://www.alphavantage.co/documentation/) and [Twelve Data](https://twelvedata.com/docs) provide financial market data APIs. This document maps the overlapping endpoints so users can provide either API key and the library can fall back to the other provider when one exceeds its rate limit.

---

## Common Market Data Endpoints

| Capability | Alpha Vantage | Twelve Data | Notes |
|---|---|---|---|
| **Time Series (Intraday)** | `TIME_SERIES_INTRADAY` (1min, 5min, 15min, 30min, 60min) | `/time_series` (1min, 5min, 15min, 30min, 45min, 1h, 2h, 4h) | Common intervals: 1min, 5min, 15min, 30min |
| **Time Series (Daily)** | `TIME_SERIES_DAILY` | `/time_series?interval=1day` | |
| **Time Series (Weekly)** | `TIME_SERIES_WEEKLY` | `/time_series?interval=1week` | |
| **Time Series (Monthly)** | `TIME_SERIES_MONTHLY` | `/time_series?interval=1month` | |
| **Quote (Real-time)** | `GLOBAL_QUOTE` | `/quote` | |
| **Symbol Search** | `SYMBOL_SEARCH` | `/symbol_search` | |
| **Exchange Rate (FX)** | `CURRENCY_EXCHANGE_RATE` | `/exchange_rate` | |

## Common Technical Indicators

Both providers expose the same set of popular technical indicator endpoints. Parameters are nearly identical (`symbol`, `interval`, `time_period`, `series_type`).

### Overlap Studies

| Indicator | Alpha Vantage | Twelve Data |
|---|---|---|
| SMA | `function=SMA` | `/sma` |
| EMA | `function=EMA` | `/ema` |
| WMA | `function=WMA` | `/wma` |
| DEMA | `function=DEMA` | `/dema` |
| TEMA | `function=TEMA` | `/tema` |
| TRIMA | `function=TRIMA` | `/trima` |
| KAMA | `function=KAMA` | `/kama` |
| T3 | `function=T3` | `/t3ma` |
| VWAP | `function=VWAP` | `/vwap` |
| BBANDS | `function=BBANDS` | `/bbands` |
| MIDPOINT | `function=MIDPOINT` | `/midpoint` |
| MIDPRICE | `function=MIDPRICE` | `/midprice` |

### Momentum Indicators

| Indicator | Alpha Vantage | Twelve Data |
|---|---|---|
| MACD | `function=MACD` | `/macd` |
| RSI | `function=RSI` | `/rsi` |
| STOCH | `function=STOCH` | `/stoch` |
| STOCHRSI | `function=STOCHRSI` | `/stochrsi` |
| WILLR | `function=WILLR` | `/willr` |
| ADX | `function=ADX` | `/adx` |
| CCI | `function=CCI` | `/cci` |
| AROON | `function=AROON` | `/aroon` |
| AROONOSC | `function=AROONOSC` | `/aroonosc` |
| MOM | `function=MOM` | `/mom` |
| ROC | `function=ROC` | `/roc` |
| ROCR | `function=ROCR` | `/rocr` |
| APO | `function=APO` | `/apo` |
| BOP | `function=BOP` | `/bop` |
| CMO | `function=CMO` | `/cmo` |
| MFI | `function=MFI` | `/mfi` |
| TRIX | `function=TRIX` | `/trix` |
| DX | `function=DX` | `/dx` |
| ULTOSC | `function=ULTOSC` | `/ultosc` |
| PLUS_DI | `function=PLUS_DI` | `/plus_di` |
| MINUS_DI | `function=MINUS_DI` | `/minus_di` |
| PLUS_DM | `function=PLUS_DM` | `/plus_dm` |
| MINUS_DM | `function=MINUS_DM` | `/minus_dm` |

### Volume Indicators

| Indicator | Alpha Vantage | Twelve Data |
|---|---|---|
| OBV | `function=OBV` | `/obv` |
| AD | `function=AD` | `/ad` |
| ADOSC | `function=ADOSC` | `/adosc` |

### Volatility Indicators

| Indicator | Alpha Vantage | Twelve Data |
|---|---|---|
| ATR | `function=ATR` | `/atr` |
| NATR | `function=NATR` | `/natr` |

### Hilbert Transform

| Indicator | Alpha Vantage | Twelve Data |
|---|---|---|
| HT_DCPERIOD | `function=HT_DCPERIOD` | `/ht_dcperiod` |
| HT_PHASOR | `function=HT_PHASOR` | `/ht_phasor` |

---

## Key Mapping Differences

| Aspect | Alpha Vantage | Twelve Data |
|---|---|---|
| **Base URL** | `https://www.alphavantage.co/query` | `https://api.twelvedata.com` |
| **API key env var** | `API_KEY_AV` | `API_KEY` |
| **Interval names** | `1min`, `5min`, `60min`, `daily`, `weekly`, `monthly` | `1min`, `5min`, `1h`, `1day`, `1week`, `1month` |
| **API key parameter** | `apikey` | `apikey` |
| **Response format** | Nested JSON with dynamic keys (`"1. open"`, `"2. high"`) | Flat JSON with `meta` + `values` array |
| **Error signaling** | `"Error Message"` key in response body | `status: "error"` with `code` and `message` fields |

### Interval Mapping

| Unified | Alpha Vantage | Twelve Data |
|---|---|---|
| 1min | `1min` | `1min` |
| 5min | `5min` | `5min` |
| 15min | `15min` | `15min` |
| 30min | `30min` | `30min` |
| 1h | `60min` | `1h` |
| 1day | `daily` | `1day` |
| 1week | `weekly` | `1week` |
| 1month | `monthly` | `1month` |

---

## Endpoints Unique to Each Provider

### Alpha Vantage Only

- **Commodities** — WTI, Brent, Gold, Silver, Natural Gas, Copper, Aluminum, Wheat, Corn, Cotton, Sugar, Coffee
- **Economic Indicators** — Real GDP, CPI, Inflation, Treasury Yield, Federal Funds Rate, Unemployment, Nonfarm Payroll, Retail Sales, Durable Goods Orders
- **Options Data** — Historical and Realtime Options
- **Alpha Intelligence** — News & Sentiments, Top Gainers/Losers, Earnings Call Transcripts
- **Fundamental Data** — Company Overview, Income Statement, Balance Sheet, Cash Flow, Earnings, IPO Calendar

### Twelve Data Only

- **End of Day Price** (`/eod`)
- **Market Movers** (`/market_movers`)
- **Company Profile** (`/profile`)
- **Statistics** (`/statistics`)
- **Earnings** (`/earnings`) and Earnings Calendar
- **Recommendations** (`/recommendations`)
- **Insider Transactions** (`/insider_transactions`)
- **Logo** (`/logo`)
- **Dividends / Splits Calendar**
- **Supertrend** and other advanced volatility indicators

---

## Recommended Build Priority

For the failover use case (try provider A, fall back to provider B on rate-limit), implement in this order:

1. **Time Series** (intraday / daily / weekly / monthly) — already partially implemented in both providers
2. **Quote** — already partially implemented in both
3. **Symbol Search** — currently only Twelve Data; add Alpha Vantage `SYMBOL_SEARCH`
4. **Exchange Rate** — both support forex pair rates
5. **SMA / EMA / RSI / MACD / BBANDS** — the most popular technical indicators


(1) adding the missing Alpha Vantage endpoints (search), (2) normalizing response types into a common ToDataFrame trait for both, and (3) building a dispatcher that tries provider A, then falls back to provider B on rate-limit errors.