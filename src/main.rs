mod client;
// #[warn(unused_imports)]
mod data;
mod market_data;
mod reference_data;
use std::error::Error;

use crate::client::Twelvedata;
use crate::data::{TimeSeriesResponse, ToDataFrame};
use crate::market_data::MarketData;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Twelvedata::new()?;

    // Example 1: Time Series
    println!("=== Time Series Example ===\n");
    let market_data = MarketData { client: &client };
    let ts_response = market_data
        .get_time_series("AAPL", "1day", Some("2024-11-01"), Some("2024-11-30"))
        .await?;

    let ts_body = ts_response.text().await?;

    // Debug: Print raw response
    // println!("Raw API Response:\n{}\n", ts_body);

    let time_series: TimeSeriesResponse = serde_json::from_str(&ts_body)?;

    println!("Status: {}", time_series.status);

    if let Some(meta) = &time_series.meta {
        println!("Symbol: {}", meta.symbol);
        println!("Interval: {}", meta.interval);
    }

    if let Some(msg) = &time_series.message {
        println!("API Message: {}", msg);
    }

    println!("Data points: {}", time_series.values.len());

    // Convert to DataFrame
    let ts_df = time_series.to_dataframe()?;
    println!("\nTime Series DataFrame:");
    println!("{}", ts_df);

    // // Example 2: EOD Price
    // println!("\n=== EOD Price Example ===\n");
    // let eod_response = market_data.get_eod("AAPL", "2024-12-03").await?;
    // let eod_body = eod_response.text().await?;
    // let eod_data: EodResponse = serde_json::from_str(&eod_body)?;

    // println!("EOD for {}: {}", eod_data.symbol, eod_data.close);

    // let eod_df = eod_data.to_dataframe()?;
    // println!("\nEOD DataFrame:");
    // println!("{}", eod_df);

    // // Example 3: Quote
    // println!("\n=== Real-time Quote Example ===\n");
    // let quote_response = market_data.get_quote("MSFT").await?;
    // let quote_body = quote_response.text().await?;
    // let quote: QuoteResponse = serde_json::from_str(&quote_body)?;

    // println!("Quote for {}: {}", quote.symbol, quote.name);
    // println!(
    //     "Price: {} | Change: {} ({}%)",
    //     quote.close, quote.change, quote.percent_change
    // );

    // let quote_df = quote.to_dataframe()?;
    // println!("\nQuote DataFrame:");
    // println!("{}", quote_df);

    Ok(())
}
