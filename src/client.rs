use std::env;

use reqwest::{Client, Response};
use thiserror::Error;
use url::Url;

pub struct Twelvedata {
    client: Client,
    api_key: String,
    pub base_url: Url,
}

#[derive(Error, Debug)]
pub enum TDError {
    #[error("URL parsing error: {0}")]
    Url(#[from] url::ParseError),

    #[error("Environment variable error: {0}")]
    Env(#[from] env::VarError),

    #[error("HTTP request error: {0}")]
    Request(#[from] reqwest::Error),

    #[error("API Response error: {0}")]
    Api(String),
}

type Result<T> = std::result::Result<T, TDError>;

impl Twelvedata {
    pub fn new() -> Result<Self> {
        dotenv::dotenv().ok();
        let api_key = env::var("API_KEY")?;
        let base_url = Url::parse("https://api.twelvedata.com")?;

        Ok(Self {
            client: Client::new(),
            api_key,
            base_url,
        })
    }

    // generic method to build urls
    pub fn build_url(&self, endpoint: &str, params: &[(&str, &str)]) -> Result<Url> {
        let mut url = self.base_url.clone();
        url.set_path(endpoint);

        let mut query_pairs = url.query_pairs_mut();
        for (key, value) in params {
            query_pairs.append_pair(key, value);
        }
        query_pairs.append_pair("apikey", &self.api_key);
        drop(query_pairs);

        Ok(url)
    }

    // From here we build the specific url

    pub fn build_symbol_search_url(&self, symbol: &str) -> Result<Url> {
        self.build_url("symbol_search", &[("symbol", symbol)])
    }

    pub fn build_time_series_url(&self, symbol: &str, interval: &str) -> Result<Url> {
        self.build_url("time_series", &[("symbol", symbol), ("interval", interval)])
    }

    pub fn build_eod_url(&self, symbol: &str, date: &str) -> Result<Url> {
        self.build_url("eod", &[("symbol", symbol), ("date", date)])
    }

    pub async fn make_request(&self, url: Url) -> Result<Response> {
        let response = self
            .client
            .get(url)
            .header("User-Agent", "tickrs/0.1.0")
            .send()
            .await?;

        if response.status().is_success() {
            Ok(response)
        } else {
            Err(TDError::Api(format!(
                "API Request failed with status: {}",
                response.status()
            )))
        }
    }
}

#[cfg(test)]
mod tests {
    use dotenv::dotenv;

    use crate::client::{Result, Twelvedata};

    #[test]
    fn test_build_url() -> Result<()> {
        dotenv().ok();

        let client = Twelvedata::new()?;
        let url = client.build_symbol_search_url("AAPL")?;
        assert!(url
            .as_str()
            .starts_with("https://api.twelvedata.com/symbol_search?symbol=AAPL&apikey="));
        Ok(())
    }

    #[test]
    fn test_build_time_series_url() -> Result<()> {
        dotenv().ok();

        let client = Twelvedata::new()?;
        let url = client.build_time_series_url("RELIANCE", "1min")?;
        assert!(url.as_str().contains("symbol=RELIANCE"));
        assert!(url.as_str().contains("interval=1min"));
        assert!(url
            .as_str()
            .starts_with("https://api.twelvedata.com/time_series?symbol=RELIANCE&interval=1min"));
        Ok(())
    }

    #[test]
    fn test_eod_url() -> Result<()> {
        dotenv().ok();

        let client = Twelvedata::new()?;
        let eod_url = client.build_eod_url("TATA", "2025-11-20")?;
        assert!(eod_url
            .as_str()
            .starts_with("https://api.twelvedata.com/eod?symbol=TATA&date=2025-11-20"));
        Ok(())
    }

    #[test]
    fn test_generic_build_url() -> Result<()> {
        dotenv().ok();

        let client = Twelvedata::new()?;
        let url = client.build_url("quote", &[("symbol", "MSFT"), ("interval", "1day")])?;
        assert!(url.as_str().contains("symbol=MSFT"));
        assert!(url.as_str().contains("interval=1day"));
        Ok(())
    }

    #[test]
    fn test_build_eod_url() -> Result<()> {
        dotenv().ok();
        let client = Twelvedata::new()?;
        let url = client.build_url("eod", &[("symbol", "L&T"), ("date", "2025-12-01")])?;
        assert!(url
            .as_str()
            .starts_with("https://api.twelvedata.com/eod?symbol=L%26T&date=2025-12-01"));

        Ok(())
    }
}
