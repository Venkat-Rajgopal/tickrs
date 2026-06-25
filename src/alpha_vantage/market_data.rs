use url::Url;

use crate::alpha_vantage::client::{AlphaVantage, Result};
use crate::alpha_vantage::data::{SymbolSearchResponse, TimeSeriesResponse};

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
    ) -> Result<TimeSeriesResponse> {
        let url = self.build_time_series_url(symbol, interval)?;
        let response = self.client.make_request(url).await?;
        let data = response.json::<TimeSeriesResponse>().await?;
        Ok(data)
    }

    pub async fn get_quote(&self, symbol: &str) -> Result<serde_json::Value> {
        let url = self.build_quote_url(symbol)?;
        let response = self.client.make_request(url).await?;
        let data = response.json::<serde_json::Value>().await?;
        Ok(data)
    }

    pub fn build_search_url(&self, keywords: &str) -> Result<Url> {
        self.client.build_url(
            "/query",
            &[("function", "SYMBOL_SEARCH"), ("keywords", keywords)],
        )
    }

    pub async fn search(&self, keywords: &str) -> Result<SymbolSearchResponse> {
        let url = self.build_search_url(keywords)?;
        let response = self.client.make_request(url).await?;
        let data = response.json::<SymbolSearchResponse>().await?;
        Ok(data)
    }
}

#[cfg(test)]
mod tests {
    use dotenv::dotenv;

    use crate::alpha_vantage::client::{AVError, AlphaVantage, Result};
    use crate::alpha_vantage::data::{SymbolSearchResponse, TimeSeriesResponse};
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

        assert_eq!(response.time_series.len(), 22);

        let jan20 = response.time_series.get("2026-01-20").unwrap();
        assert_eq!(jan20.open, "301.3500");
        assert_eq!(jan20.close, "291.3500");
        assert_eq!(jan20.volume, "7211706");

        assert_eq!(response.time_series.get("2026-01-16").unwrap().close, "305.6700");
        assert_eq!(response.time_series.get("2025-08-29").unwrap().close, "243.4900");

        Ok(())
    }

    #[test]
    fn test_build_search_url() -> Result<()> {
        let client = setup()?;
        let md = AlphaVantageMarket { client: &client };
        let res_url = md.build_search_url("tesco")?;

        let url_str = res_url.as_str();
        assert!(url_str.starts_with("https://www.alphavantage.co/query?"));
        assert!(url_str.contains("function=SYMBOL_SEARCH"));
        assert!(url_str.contains("keywords=tesco"));
        assert!(url_str.contains("apikey="));

        Ok(())
    }

    #[test]
    fn test_search_response_deserialization() -> Result<()> {
        let mock_json = r#"{
            "bestMatches": [
                {
                    "1. symbol": "TSCO.LON",
                    "2. name": "Tesco PLC",
                    "3. type": "Equity",
                    "4. region": "United Kingdom",
                    "5. marketOpen": "08:00",
                    "6. marketClose": "16:30",
                    "7. timezone": "UTC+01",
                    "8. currency": "GBX",
                    "9. matchScore": "0.7273"
                }
            ]
        }"#;

        let response: SymbolSearchResponse = serde_json::from_str(mock_json)
            .map_err(|e| AVError::Api(format!("JSON parse error: {}", e)))?;

        assert_eq!(response.best_matches.len(), 1);
        assert_eq!(response.best_matches[0].symbol, "TSCO.LON");
        assert_eq!(response.best_matches[0].name, "Tesco PLC");
        assert_eq!(response.best_matches[0].instrument_type, "Equity");
        assert_eq!(response.best_matches[0].region, "United Kingdom");
        assert_eq!(response.best_matches[0].currency, "GBX");

        Ok(())
    }
}
