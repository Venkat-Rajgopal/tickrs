[![CI](https://github.com/Venkat-Rajgopal/tickrs/actions/workflows/ci.yml/badge.svg)](https://github.com/Venkat-Rajgopal/tickrs/actions/workflows/ci.yml)

```
 ██   ▀▀        ▄▄                   
▀██▀▀ ██  ▄████ ██ ▄█▀ ████▄ ▄█▀▀▀   
 ██   ██  ██    ████   ██ ▀▀ ▀███▄  
 ██   ██▄ ▀████ ██ ▀█▄ ██    ▄▄▄█▀  
                              
```

# tickrs :crab:

A Rust toolkit for financial market data built around [TwelveData](https://twelvedata.com/docs) and [Alpha Vantage](https://www.alphavantage.co/documentation/).

> **Note:** :construction: Early-stage project — features and APIs are still evolving.

## Services

| Service | Status | Description |
|---------|--------|-------------|
| **CLI** | Active | Command-line interface for querying market data |
| **Clients** | Active | Type-safe Rust clients for TwelveData and Alpha Vantage |
| **TUI** | Planned | Terminal UI with charts |

## CLI

See [CLI.md](CLI.md) for the full command reference.

```bash
# Symbol search
cargo run -- search AAPL
cargo run -- -p av search AAPL

# Time series (OHLCV) — intervals: 1min 5min 15min 30min 1h 1day 1week 1month
cargo run -- time-series AAPL -i 1day
cargo run -- time-series AAPL -i 1day -s 2025-11-01 -e 2025-11-30
cargo run -- -p av time-series AAPL -i 1week

# End-of-day closing price (TwelveData only)
cargo run -- eod AAPL 2025-12-01

# Real-time quote
cargo run -- quote AAPL
cargo run -- -p av quote AAPL
```

## Clients

```rust
// TwelveData
let client = Twelvedata::new()?;
let market = MarketData { client: &client };
let df = market.get_time_series_data("AAPL", "1day", None, None).await?.to_dataframe()?;

// Alpha Vantage
let client = AlphaVantage::new()?;
let market = MarketData { client: &client };
let df = market.get_time_series_data("AAPL", "1day").await?.to_dataframe()?;
```

## Setup

```
API_KEY=<TwelveData API key>
API_KEY_AV=<Alpha Vantage API key>
```
