# TwelveData CLI Tool

A command-line interface for querying the TwelveData API.

## Setup

Ensure your API key is set in the `.env` file:
```
API_KEY=your_api_key_here
```

## Usage

### Symbol Search
Search for symbols across exchanges:
```bash
cargo run --bin cli search AAPL
cargo run --bin cli search RELIANCE
```

### Time Series Data
Get historical price data:
```bash
# Default (1 day interval, all available data)
cargo run --bin cli time-series AAPL

# With custom interval
cargo run --bin cli time-series AAPL --interval 1h

# With date range
cargo run --bin cli time-series AAPL --start 2025-11-01 --end 2025-11-15

# Short form
cargo run --bin cli time-series AAPL -s 2025-11-01 -e 2025-11-15
```

Available intervals: `1min`, `5min`, `15min`, `30min`, `1h`, `4h`, `1day`, `1week`, `1month`

### End-of-Day Price
Get the closing price for a specific date:
```bash
cargo run --bin cli eod AAPL 2025-12-01
cargo run --bin cli eod RELIANCE 2025-11-30
```

### Real-time Quote
Get current market data:
```bash
cargo run --bin cli quote AAPL
cargo run --bin cli quote MSFT
```

## Examples Output

All commands return data as nicely formatted Polars DataFrames for easy viewing.

## Building for Production

Build an optimized binary:
```bash
cargo build --release --bin cli
./target/release/cli quote AAPL
```

## Help

Get help for any command:
```bash
cargo run --bin cli --help
cargo run --bin cli time-series --help
```
