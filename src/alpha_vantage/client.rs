use std::env;

use reqwest::{Client, Response};
use thiserror::Error;
use url::Url;

pub type Result<T> = std::result::Result<T, AVError>;

pub struct AlphaVantage {
    client: Client,
    api_key: String,
    pub base_url: Url,
}

#[derive(Error, Debug)]
pub enum AVError {
    #[error("URL parsing error: {0}")]
    Url(#[from] url::ParseError),

    #[error("Environment variable error: {0}")]
    Env(#[from] env::VarError),

    #[error("HTTP request error: {0}")]
    Request(#[from] reqwest::Error),

    #[error("API Response error: {0}")]
    Api(String),
}

impl AlphaVantage {
    pub fn new() -> Result<Self> {
        dotenv::dotenv().ok();
        let api_key = env::var("API_KEY_AV")?;
        let base_url = Url::parse("https://www.alphavantage.co")?;

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
            Err(AVError::Api(format!(
                "API Request failed with status: {}",
                response.status()
            )))
        }
    }
}

#[cfg(test)]
impl AlphaVantage {
    pub fn test_client(base_url: &str) -> Result<Self> {
        use reqwest::Client;
        use url::Url;

        Ok(Self {
            client: Client::new(),
            api_key: "test_api_key".to_string(),
            base_url: Url::parse(base_url)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{AlphaVantage, Result};

    #[test]
    fn test_build_url() -> Result<()> {
        let client = AlphaVantage::test_client("https://www.alphavantage.co")?;
        let url = client.build_url(
            "/query",
            &[("function", "GLOBAL_QUOTE"), ("symbol", "AAPL")],
        )?;
        let params: std::collections::HashMap<_, _> = url.query_pairs().collect();
        assert_eq!(url.scheme(), "https");
        assert_eq!(url.host_str(), Some("www.alphavantage.co"));
        assert_eq!(url.path(), "/query");
        assert_eq!(
            params.get("function").map(|s| s.as_ref()),
            Some("GLOBAL_QUOTE")
        );
        assert_eq!(params.get("symbol").map(|s| s.as_ref()), Some("AAPL"));
        assert!(params.contains_key("apikey"));
        Ok(())
    }
}
