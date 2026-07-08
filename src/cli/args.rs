use clap::{Parser, Subcommand, ValueEnum};

#[derive(ValueEnum, Clone, Debug)]
pub enum Provider {
    /// TwelveData (default)
    #[value(name = "12d")]
    TwelveData,
    /// Alpha Vantage
    #[value(name = "av")]
    AlphaVantage,
}

#[derive(ValueEnum, Clone, Debug, Default)]
pub enum Interval {
    #[value(name = "1min")]
    OneMin,
    #[value(name = "5min")]
    FiveMin,
    #[value(name = "15min")]
    FifteenMin,
    #[value(name = "30min")]
    ThirtyMin,
    #[value(name = "1h")]
    OneHour,
    #[default]
    #[value(name = "1day")]
    OneDay,
    #[value(name = "1week")]
    OneWeek,
    #[value(name = "1month")]
    OneMonth,
}

impl Interval {
    /// API string for TwelveData
    pub fn as_twelvedata(&self) -> &'static str {
        match self {
            Interval::OneMin => "1min",
            Interval::FiveMin => "5min",
            Interval::FifteenMin => "15min",
            Interval::ThirtyMin => "30min",
            Interval::OneHour => "1h",
            Interval::OneDay => "1day",
            Interval::OneWeek => "1week",
            Interval::OneMonth => "1month",
        }
    }

    /// API string for Alpha Vantage (intraday uses minute notation)
    pub fn as_alphavantage(&self) -> &'static str {
        match self {
            Interval::OneMin => "1min",
            Interval::FiveMin => "5min",
            Interval::FifteenMin => "15min",
            Interval::ThirtyMin => "30min",
            Interval::OneHour => "60min",
            Interval::OneDay => "1day",
            Interval::OneWeek => "1week",
            Interval::OneMonth => "1month",
        }
    }
}

#[derive(Parser)]
#[command(name = "tickrs")]
#[command(about = "tickrs 🦀 — financial data CLI", long_about = None)]
pub struct Cli {
    /// Data provider
    #[arg(short, long, default_value = "12d", global = true)]
    pub provider: Provider,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Search for symbols by keyword
    Search {
        /// Keyword to search (e.g., AAPL, Tesla)
        symbol: String,
    },
    /// Get OHLCV time series data
    TimeSeries {
        /// Ticker symbol (e.g., AAPL)
        symbol: String,
        /// Bar interval
        #[arg(short, long, default_value = "1day")]
        interval: Interval,
        /// Start date, TwelveData only (YYYY-MM-DD)
        #[arg(short, long)]
        start: Option<String>,
        /// End date, TwelveData only (YYYY-MM-DD)
        #[arg(short, long)]
        end: Option<String>,
    },
    /// Get end-of-day closing price (TwelveData only)
    Eod {
        /// Ticker symbol (e.g., AAPL)
        symbol: String,
        /// Date (YYYY-MM-DD)
        date: String,
    },
    /// Get real-time quote
    Quote {
        /// Ticker symbol (e.g., AAPL)
        symbol: String,
    },
}
