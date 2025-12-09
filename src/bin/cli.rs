use clap::{Parser, Subcommand};
use tickrs::client::Twelvedata;
use tickrs::data::ToDataFrame;
use tickrs::market_data::MarketData;
use tickrs::reference_data::ReferenceData;

#[derive(Parser)]
#[command(name = "tickrs")]
#[command(about = "TwelveData API CLI Tool", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Search for symbols
    Search {
        /// Symbol to search for (e.g., AAPL, RELIANCE)
        symbol: String,
    },
    /// Get time series data
    TimeSeries {
        /// Symbol (e.g., AAPL)
        symbol: String,
        /// Interval (e.g., 1min, 5min, 1h, 1day, 1week, 1month)
        #[arg(short, long, default_value = "1day")]
        interval: String,
        /// Start date (YYYY-MM-DD)
        #[arg(short, long)]
        start: Option<String>,
        /// End date (YYYY-MM-DD)
        #[arg(short, long)]
        end: Option<String>,
    },
    /// Get end-of-day price
    Eod {
        /// Symbol (e.g., AAPL)
        symbol: String,
        /// Date (YYYY-MM-DD)
        date: String,
    },
    /// Get real-time quote
    Quote {
        /// Symbol (e.g., AAPL)
        symbol: String,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    // Initialize the client
    let client = Twelvedata::new()?;

    match cli.command {
        Commands::Search { symbol } => {
            let reference = ReferenceData { client: &client };
            let response = reference.get_symbol_search(&symbol).await?;

            println!("Status: {}", response.status);
            println!("Found {} results\n", response.data.len());

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
            let market = MarketData { client: &client };
            let response = market
                .get_time_series_data(&symbol, &interval, start.as_deref(), end.as_deref())
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
            println!(
                "\nChange: {} ({}%)",
                response.change, response.percent_change
            );
            println!("Volume: {}\n", response.volume);

            match response.to_dataframe() {
                Ok(df) => println!("{}", df),
                Err(e) => eprintln!("Error creating DataFrame: {}", e),
            }
        }
    }

    Ok(())
}
