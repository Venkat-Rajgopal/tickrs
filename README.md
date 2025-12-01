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

```rust
use twelvedata::Twelvedata;

let client = Twelvedata::new()?;
let url = client.build_time_series_url("AAPL", "1min")?;
let response = client.make_request(url).await?;
```

