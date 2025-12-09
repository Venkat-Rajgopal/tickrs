use reqwest::Response;
use url::Url;

use crate::client::{Result, Twelvedata};
use crate::data::{EodResponse, QuoteResponse, TimeSeriesResponse};

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

    pub async fn get_time_series(
        &self,
        symbol: &str,
        interval: &str,
        start_date: Option<&str>,
        end_date: Option<&str>,
    ) -> Result<Response> {
        let url = self.build_time_series_url(symbol, interval, start_date, end_date)?;
        self.client.make_request(url).await
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

    pub async fn get_eod(&self, symbol: &str, date: &str) -> Result<EodResponse> {
        let url = self.build_eod_url(symbol, date)?;
        let response = self.client.make_request(url).await?;
        let data = response.json::<EodResponse>().await?;
        Ok(data)
    }

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

    use crate::{
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
}
