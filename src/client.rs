use std::env;

use reqwest::{Client, Response};
use thiserror::Error;
use url::Url;

pub type Result<T> = std::result::Result<T, TDError>;

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
    use crate::client::{Result, Twelvedata};
    use dotenv::dotenv;

    impl Twelvedata {
        pub fn test_client(base_url: &str) -> Result<Self> {
            use reqwest::Client;
            use url::Url;

            let api_key = "test_api_key".to_string();
            let base_url = Url::parse(base_url)?;

            Ok(Self {
                client: Client::new(),
                api_key,
                base_url,
            })
        }
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
