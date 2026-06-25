use crate::cli::args::Commands;
use crate::twelvedata::client::Twelvedata;
use crate::twelvedata::data::ToDataFrame;
use crate::twelvedata::market_data::MarketData;
use crate::twelvedata::reference_data::ReferenceData;

pub async fn run(command: Commands) -> Result<(), Box<dyn std::error::Error>> {
    let client = Twelvedata::new()?;

    match command {
        Commands::Search { symbol } => {
            let reference = ReferenceData { client: &client };
            let response = reference.get_symbol_search(&symbol).await?;
            println!("Status: {}", response.status);
            match response.to_dataframe() {
                Ok(df) => println!("{}", df),
                Err(e) => eprintln!("Error creating DataFrame: {}", e),
            }
        }
        Commands::TimeSeries { symbol, interval, start, end } => {
            let market = MarketData { client: &client };
            let response = market
                .get_time_series_data(&symbol, interval.as_twelvedata(), start.as_deref(), end.as_deref())
                .await?;
            println!("Status: {}", response.status);
            if let Some(meta) = &response.meta {
                println!("Symbol: {} ({})", meta.symbol, meta.exchange);
                println!("Interval: {}", meta.interval);
            }
            println!("Data points: {}\n", response.values.len());
            match response.to_dataframe() {
                Ok(df) => println!("{}", df),
                Err(e) => eprintln!("Error creating DataFrame: {}", e),
            }
        }
        Commands::Eod { symbol, date } => {
            let market = MarketData { client: &client };
            let response = market.get_eod(&symbol, &date).await?;
            match response.to_dataframe() {
                Ok(df) => println!("{}", df),
                Err(e) => eprintln!("Error creating DataFrame: {}", e),
            }
        }
        Commands::Quote { symbol } => {
            let market = MarketData { client: &client };
            let response = market.get_quote(&symbol).await?;
            println!("Symbol: {} - {}", response.symbol, response.name);
            println!("Exchange: {} ({})", response.exchange, response.mic_code);
            println!("Time: {}", response.datetime);
            println!("\nPrice Data:");
            println!("  Open:  {} {}", response.open, response.currency);
            println!("  High:  {} {}", response.high, response.currency);
            println!("  Low:   {} {}", response.low, response.currency);
            println!("  Close: {} {}", response.close, response.currency);
            println!("\nChange: {} ({}%)", response.change, response.percent_change);
            println!("Volume: {}\n", response.volume);
            match response.to_dataframe() {
                Ok(df) => println!("{}", df),
                Err(e) => eprintln!("Error creating DataFrame: {}", e),
            }
        }
    }

    Ok(())
}
