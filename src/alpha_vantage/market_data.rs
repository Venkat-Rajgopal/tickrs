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

#[cfg(test)]
mod tests {
    use dotenv::dotenv;

    use crate::alpha_vantage::client::{AlphaVantage, Result};
    use crate::alpha_vantage::data::TimeSeriesResponse;
    use crate::alpha_vantage::market_data::MarketData as AlphaVantageMarket;

    fn setup() -> Result<AlphaVantage> {
        dotenv().ok();
        AlphaVantage::new()
    }

    #[test]
    fn test_build_time_series_url() -> Result<()> {
        let client = setup()?;
        let md = AlphaVantageMarket { client: &client };
        let res_url = md.build_time_series_url("AAPL", "1day")?;

        let url_str = res_url.as_str();
        assert!(url_str.starts_with("https://www.alphavantage.co/query?"));
        assert!(url_str.contains("function=TIME_SERIES_DAILY"));
        assert!(url_str.contains("symbol=AAPL"));
        assert!(url_str.contains("apikey="));

        Ok(())
    }

    #[test]
    fn test_time_series_response_deserialization() -> Result<()> {
        // Load the mock response from time_series_av.json
        let mock_json = include_str!("../../resources/time_series_av.json");

        // Parse the JSON - this simulates what get_time_series_data does
        // after receiving the HTTP response
        let response: TimeSeriesResponse = serde_json::from_str(mock_json).map_err(|e| {
            crate::alpha_vantage::client::AVError::Api(format!("JSON parse error: {}", e))
        })?;

        // Assert the response matches what we expect from time_series_av.json
        assert_eq!(response.meta_data.symbol, "IBM");
        assert_eq!(
            response.meta_data.information,
            "Weekly Prices (open, high, low, close) and Volumes"
        );
        assert_eq!(response.meta_data.last_refreshed, "2026-01-20");
        assert_eq!(response.meta_data.time_zone, "US/Eastern");

        // Verify we have time series data (22 weeks in the mock file)
        assert_eq!(response.weekly_time_series.len(), 22);

        // Verify specific close values
        assert_eq!(
            response.weekly_time_series.get("2026-01-20").unwrap().close,
            "291.3500"
        );
        assert_eq!(
            response.weekly_time_series.get("2026-01-16").unwrap().close,
            "305.6700"
        );
        assert_eq!(
            response.weekly_time_series.get("2025-08-29").unwrap().close,
            "243.4900"
        );

        Ok(())
    }
}
