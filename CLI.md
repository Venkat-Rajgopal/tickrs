# tickrs CLI

A command-line interface for querying financial market data from TwelveData and Alpha Vantage.

## Setup

Create a `.env` file in the project root:
```
API_KEY=<TwelveData API key>
API_KEY_AV=<Alpha Vantage API key>
```

## Providers

Use `-p` / `--provider` to select the data source (default: `12d`):

| Flag | Provider |
|------|----------|
| `-p 12d` | TwelveData (default) |
| `-p av` | Alpha Vantage |

## Commands

### Symbol Search
```bash
cargo run -- search AAPL
cargo run -- search Tesla

# Alpha Vantage
cargo run -- -p av search AAPL
```

### Time Series (OHLCV)
```bash
# Default: TwelveData, 1day interval
cargo run -- time-series AAPL

# With interval (1min 5min 15min 30min 1h 1day 1week 1month)
cargo run -- time-series AAPL -i 1week

# With date range (TwelveData only)
cargo run -- time-series AAPL -i 1day -s 2025-11-01 -e 2025-11-30

# Alpha Vantage
cargo run -- -p av time-series AAPL -i 1week
```

### End-of-Day Price (TwelveData only)
```bash
cargo run -- eod AAPL 2025-12-01
```

### Real-time Quote
```bash
cargo run -- quote AAPL

# Alpha Vantage
cargo run -- -p av quote AAPL
```

## Help

```bash
cargo run -- --help
cargo run -- time-series --help
```

## Release Build

```bash
cargo build --release
./target/release/tickrs -p av time-series AAPL -i 1day
```
