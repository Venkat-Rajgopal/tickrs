mod alpha_vantage;
mod twelvedata;

use std::error::Error;

// use crate::alpha_vantage::client::AlphaVantage;
use crate::twelvedata::client::Twelvedata;
use crate::twelvedata::data::ToDataFrame;
use crate::twelvedata::market_data::MarketData;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Twelvedata::new()?;

    let market_data = MarketData { client: &client };
    let ts_response = market_data
        .get_time_series_data("AAPL", "1day", Some("2025-12-10"), Some("2025-12-15"))
        .await?;

    match ts_response.to_dataframe() {
        Ok(df) => println!("{}", df),
        Err(e) => eprintln!("Error creating dataframe: {}", e),
    }

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
