use crate::alpha_vantage::client::AlphaVantage;
use crate::alpha_vantage::data::ToDataFrame;
use crate::alpha_vantage::market_data::MarketData;
use crate::cli::args::Commands;

pub async fn run(command: Commands) -> Result<(), Box<dyn std::error::Error>> {
    let client = AlphaVantage::new()?;
    let market = MarketData { client: &client };

    match command {
        Commands::Search { symbol } => {
            let response = market.search(&symbol).await?;
            match response.to_dataframe() {
                Ok(df) => println!("{}", df),
                Err(e) => eprintln!("Error creating DataFrame: {}", e),
            }
        }
        Commands::TimeSeries {
            symbol,
            interval,
            start,
            end,
        } => {
            if start.is_some() || end.is_some() {
                eprintln!("Note: Alpha Vantage does not support --start/--end date filters.");
            }
            let response = market
                .get_time_series_data(&symbol, interval.as_alphavantage())
                .await?;
            match response.to_dataframe() {
                Ok(df) => println!("{}", df),
                Err(e) => eprintln!("Error creating DataFrame: {}", e),
            }
        }
        Commands::Eod { .. } => {
            eprintln!(
                "Error: 'eod' is not supported by Alpha Vantage. Use --provider 12d instead."
            );
            std::process::exit(1);
        }
        Commands::Quote { symbol } => {
            let response = market.get_quote(&symbol).await?;
            match response.to_dataframe() {
                Ok(df) => println!("{}", df),
                Err(e) => eprintln!("Error creating DataFrame: {}", e),
            }
        }
    }

    Ok(())
}
