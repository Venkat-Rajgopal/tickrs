# tickrs :crab:

A Rust client for the `TwelveData` API and others eventually, designed for efficient time series data analysis and financial market insights.

>NOTE: :construction: This is an early-stage project and is still under development. Features and APIs will evolve. 

## Overview

`tickrs` provides a type-safe, ergonomic interface to interact with TwelveData's financial market API. Built with performance in mind, it leverages `Polars` for fast DataFrame operations on time series data.

## Features

- **Flexible URL Builder** - Generic endpoint support with typed convenience methods
- **Polars Integration** - Convert API responses directly to DataFrames for analysis

## Roadmap :construction:

- [ ] Time series data retrieval and analysis
- [ ] Polars transformations and aggregations
- [ ] Historical data caching
- [ ] Real-time data streaming support

## Usage

### Market Data

```rust
use tickrs::client::Twelvedata;
use tickrs::marketdata::MarketData;

#[tokio::main]
async fn main() -> Result<()> {
    let client = Twelvedata::new()?;
    let market_data = MarketData { client: &client };
    
    // Get time series data with date range
    let response = market_data.get_time_series(
        "AAPL",
        "1day",
        Some("2025-11-01"),
        Some("2025-11-30")
    ).await?;
    
    let data: TimeSeriesResponse = response.json().await?;
    let df = data.to_dataframe()?;
    println!("{:?}", df);
    
    Ok(())
}
```

### Reference Data

```rust
use tickrs::client::Twelvedata;
use tickrs::reference_data::ReferenceData;

#[tokio::main]
async fn main() -> Result<()> {
    let client = Twelvedata::new()?;
    let reference_data = ReferenceData { client: &client };
    
    // Search for symbols
    let url = reference_data.get_symbol_search_url("RELIANCE")?;
    println!("Symbol search URL: {}", url);
    
    Ok(())
}
```

