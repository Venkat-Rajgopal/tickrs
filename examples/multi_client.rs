use std::error::Error;
use std::time::Duration;
use tokio::time::sleep;

// Example showing how to use either Twelvedata or AlphaVantage
// Both follow the same pattern!

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Option 1: Using Twelvedata
    println!("=== Using Twelvedata ===\n");
    use tickrs::twelvedata::client::Twelvedata;
    use tickrs::twelvedata::data::ToDataFrame;
    use tickrs::twelvedata::market_data::MarketData as TwelvedataMarket;

    let td_client = Twelvedata::new()?;
    let td_market = TwelvedataMarket { client: &td_client };

    let ts_response = td_market
        .get_time_series_data("AAPL", "1day", Some("2025-12-10"), Some("2025-12-15"))
        .await?;

    match ts_response.to_dataframe() {
        Ok(df) => println!("Twelvedata DataFrame:\n{}\n", df),
        Err(e) => eprintln!("Error: {}", e),
    }

    // Option 2: Using AlphaVantage (same pattern!)
    println!("=== Using AlphaVantage ===\n");
    use tickrs::alpha_vantage::client::AlphaVantage;
    use tickrs::alpha_vantage::market_data::MarketData as AlphaVantageMarket;

    let av_client = AlphaVantage::new()?;
    let av_market = AlphaVantageMarket { client: &av_client };

    // let av_data = av_market.get_time_series_data("AAPL", "1day").await?;
    // println!("AlphaVantage response:\n{:#?}\n", av_data);

    let av_quote = av_market.get_quote("AAPL").await?;
    println!("AlphaVantage quote:\n{:#?}", av_quote);

    // AlphaVantage free tier: 1 request per second limit
    println!("\nWaiting 1 second for rate limit...");
    sleep(Duration::from_secs(1)).await;

    let search_response = av_market.search("tesco").await?;
    let matches = search_response.best_matches;
    println!("Matching results for tesco {:?}", matches);

    Ok(())
}
