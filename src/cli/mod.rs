mod alphavantage;
pub mod args;
mod twelvedata;

use args::{Cli, Provider};
use clap::Parser;

pub async fn run_cli() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    std::env::set_var("POLARS_FMT_MAX_ROWS", "100");

    match cli.provider {
        Provider::TwelveData => twelvedata::run(cli.command).await,
        Provider::AlphaVantage => alphavantage::run(cli.command).await,
    }
}
