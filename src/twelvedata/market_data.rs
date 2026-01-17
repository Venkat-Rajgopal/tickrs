use url::Url;

use crate::twelvedata::client::{Result, Twelvedata};
use crate::twelvedata::data::{EodResponse, QuoteResponse, TimeSeriesResponse};

// Holds a reference to the TwelveData client
pub struct MarketData<'a> {
    pub client: &'a Twelvedata,
}

impl<'a> MarketData<'a> {
    pub fn build_time_series_url(
        &self,
        symbol: &str,
        interval: &str,
        start_date: Option<&str>,
        end_date: Option<&str>,
    ) -> Result<Url> {
        let mut params = vec![("symbol", symbol), ("interval", interval)];

        if let Some(start) = start_date {
            params.push(("start_date", start));
        }

        if let Some(end) = end_date {
            params.push(("end_date", end));
        }

        self.client.build_url("time_series", &params)
    }

    pub fn build_eod_url(&self, symbol: &str, date: &str) -> Result<Url> {
        self.client
            .build_url("eod", &[("symbol", symbol), ("date", date)])
    }

    pub fn build_quote_url(&self, symbol: &str) -> Result<Url> {
        self.client.build_url("quote", &[("symbol", symbol)])
    }

    pub async fn get_time_series_data(
        &self,
        symbol: &str,
        interval: &str,
        start_date: Option<&str>,
        end_date: Option<&str>,
    ) -> Result<TimeSeriesResponse> {
        let url = self.build_time_series_url(symbol, interval, start_date, end_date)?;
        let response = self.client.make_request(url).await?;
        let data = response.json::<TimeSeriesResponse>().await?;
        Ok(data)
    }

    #[allow(dead_code)]
    pub async fn get_eod(&self, symbol: &str, date: &str) -> Result<EodResponse> {
        let url = self.build_eod_url(symbol, date)?;
        let response = self.client.make_request(url).await?;
        let data = response.json::<EodResponse>().await?;
        Ok(data)
    }

    #[allow(dead_code)]
    pub async fn get_quote(&self, symbol: &str) -> Result<QuoteResponse> {
        let url = self.build_quote_url(symbol)?;
        let response = self.client.make_request(url).await?;
        let data = response.json::<QuoteResponse>().await?;
        Ok(data)
    }
}

#[cfg(test)]
mod tests {
    use dotenv::dotenv;

    use crate::twelvedata::{
        client::{Result, Twelvedata},
        market_data::MarketData,
    };

    #[test]
    fn test_time_series_url_withstart() -> Result<()> {
        dotenv().ok();
        let client = Twelvedata::new()?;
        let market_data = MarketData { client: &client };
        let ts_url = market_data.build_time_series_url(
            "RELIANCE",
            "1day",
            Some("2025-11-01"),
            Some("2025-11-15"),
        )?;

        // Check scheme, host, and path
        assert_eq!(ts_url.scheme(), "https");
        assert_eq!(ts_url.host_str(), Some("api.twelvedata.com"));
        assert_eq!(ts_url.path(), "/time_series");

        // Check query parameters
        let query_params: std::collections::HashMap<_, _> = ts_url.query_pairs().collect();
        assert_eq!(
            query_params.get("symbol").map(|s| s.as_ref()),
            Some("RELIANCE")
        );
        assert_eq!(
            query_params.get("interval").map(|s| s.as_ref()),
            Some("1day")
        );
        assert_eq!(
            query_params.get("start_date").map(|s| s.as_ref()),
            Some("2025-11-01")
        );
        assert_eq!(
            query_params.get("end_date").map(|s| s.as_ref()),
            Some("2025-11-15")
        );
        assert!(query_params.contains_key("apikey"));

        Ok(())
    }

    #[test]
    fn test_time_series_url() -> Result<()> {
        dotenv().ok();
        let client = Twelvedata::new()?;
        let market_data = MarketData { client: &client };
        let ts_url = market_data.build_time_series_url("RELIANCE", "1day", None, None)?;

        // Check scheme, host, and path
        assert_eq!(ts_url.scheme(), "https");
        assert_eq!(ts_url.host_str(), Some("api.twelvedata.com"));
        assert_eq!(ts_url.path(), "/time_series");

        // Check query parameters
        let query_params: std::collections::HashMap<_, _> = ts_url.query_pairs().collect();
        assert_eq!(
            query_params.get("symbol").map(|s| s.as_ref()),
            Some("RELIANCE")
        );
        assert_eq!(
            query_params.get("interval").map(|s| s.as_ref()),
            Some("1day")
        );
        assert!(query_params.contains_key("apikey"));

        Ok(())
    }

    #[test]
    fn test_eod_url() -> Result<()> {
        dotenv().ok();

        let client = Twelvedata::new()?;
        let market_data = MarketData { client: &client };
        let eod_url = market_data.build_eod_url("TATA", "2025-11-20")?;
        assert!(eod_url
            .as_str()
            .starts_with("https://api.twelvedata.com/eod?symbol=TATA&date=2025-11-20"));
        Ok(())
    }

    // Test that validates the JSON deserialization using time_series.json
    // This tests that get_time_series_data would correctly parse the API response
    #[test]
    fn test_time_series_response_deserialization() -> Result<()> {
        use crate::twelvedata::data::TimeSeriesResponse;

        // Load the mock response from time_series.json
        let mock_json = include_str!("../../resources/time_series.json");

        // Parse the JSON - this simulates what get_time_series_data does
        // after receiving the HTTP response
        let result: TimeSeriesResponse = serde_json::from_str(mock_json).map_err(|e| {
            crate::twelvedata::client::TDError::Api(format!("JSON parse error: {}", e))
        })?;

        // Assert the response matches what we expect from time_series.json
        assert_eq!(result.status, "ok");
        assert!(result.meta.is_some());

        let meta = result.meta.unwrap();
        assert_eq!(meta.symbol, "AAPL");
        assert_eq!(meta.interval, "1min");
        assert_eq!(meta.currency, "USD");
        assert_eq!(meta.exchange, "NASDAQ");

        // Check that we have the expected number of values
        assert_eq!(result.values.len(), 30);

        // Check the first value
        let first_value = &result.values[0];
        assert_eq!(first_value.datetime, "2025-12-12 15:59:00");
        assert_eq!(first_value.open, "278.14999");
        assert_eq!(first_value.close, "278.37000");
        assert_eq!(first_value.volume, Some("383024".to_string()));

        Ok(())
    }

    #[tokio::test]
    async fn test_get_time_series_data() -> Result<()> {
        use httpmock::prelude::*;

        // Start a mock HTTP server
        let server = MockServer::start();

        // Load the mock response from time_series.json
        let mock_json = include_str!("../../resources/time_series.json");

        // Configure the mock to return our test data when /time_series is called
        let _mock = server.mock(|when, then| {
            when.method(GET).path("/time_series");
            then.status(200)
                .header("content-type", "application/json")
                .body(mock_json);
        });

        // Create a test client pointing to our mock server
        let client = Twelvedata::test_client(&server.base_url())?;
        let market_data = MarketData { client: &client };

        // Call get_time_series_data - it will use the mock server instead of real API
        let result = market_data
            .get_time_series_data("AAPL", "1min", None, None)
            .await?;

        // Verify the response matches time_series.json
        assert_eq!(result.status, "ok");
        assert!(result.meta.is_some());

        let meta = result.meta.unwrap();
        assert_eq!(meta.symbol, "AAPL");
        assert_eq!(meta.interval, "1min");
        assert_eq!(result.values.len(), 30);

        Ok(())
    }
}
