use url::Url;

use crate::alpha_vantage::client::{AlphaVantage, Result};

// Holds a reference to the AlphaVantage client
pub struct MarketData<'a> {
    pub client: &'a AlphaVantage,
}

impl<'a> MarketData<'a> {
    pub fn build_time_series_url(&self, symbol: &str, interval: &str) -> Result<Url> {
        let function = match interval {
            "1min" => "TIME_SERIES_INTRADAY",
            "5min" => "TIME_SERIES_INTRADAY",
            "15min" => "TIME_SERIES_INTRADAY",
            "30min" => "TIME_SERIES_INTRADAY",
            "60min" => "TIME_SERIES_INTRADAY",
            "1day" => "TIME_SERIES_DAILY",
            "1week" => "TIME_SERIES_WEEKLY",
            "1month" => "TIME_SERIES_MONTHLY",
            _ => "TIME_SERIES_DAILY",
        };

        let mut params = vec![("function", function), ("symbol", symbol)];

        // Add interval for intraday data
        if function == "TIME_SERIES_INTRADAY" {
            params.push(("interval", interval));
        }

        self.client.build_url("/query", &params)
    }

    pub fn build_quote_url(&self, symbol: &str) -> Result<Url> {
        self.client.build_url(
            "/query",
            &[("function", "GLOBAL_QUOTE"), ("symbol", symbol)],
        )
    }

    pub async fn get_time_series_data(
        &self,
        symbol: &str,
        interval: &str,
    ) -> Result<serde_json::Value> {
        let url = self.build_time_series_url(symbol, interval)?;
        let response = self.client.make_request(url).await?;
        let data = response.json::<serde_json::Value>().await?;
        Ok(data)
    }

    pub async fn get_quote(&self, symbol: &str) -> Result<serde_json::Value> {
        let url = self.build_quote_url(symbol)?;
        let response = self.client.make_request(url).await?;
        let data = response.json::<serde_json::Value>().await?;
        Ok(data)
    }
}
