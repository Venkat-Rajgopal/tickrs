mod client;
// #[warn(unused_imports)]
mod data;

use std::error::Error;

use crate::client::Twelvedata;
use crate::data::{SymbolSearchResponse, ToDataFrame};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Twelvedata::new()?;

    // Example 1: Symbol search using convenience method
    let symbol_url = client.build_symbol_search_url("RELIANCE")?;
    let response = client.make_request(symbol_url).await?;
    let body = response.text().await?;
    let symbol_response: SymbolSearchResponse = serde_json::from_str(&body)?;

    println!("Status: {}", symbol_response.status);
    println!("Found {} instruments:", symbol_response.data.len());

    let df = symbol_response.to_dataframe()?;
    println!("\nDataFrame:");
    println!("{}", df);

    // Example 2: Time series using convenience method
    println!("\n=== Time Series Example ===");
    let time_series_url = client.build_time_series_url("AAPL", "1min")?;
    println!("Time series URL: {}", time_series_url);

    // Example 3: Generic URL builder for any endpoint
    println!("\n=== Generic URL Builder ===");
    let quote_url = client.build_url("quote", &[("symbol", "MSFT")])?;
    println!("Quote URL: {}", quote_url);

    Ok(())
}
