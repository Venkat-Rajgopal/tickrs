use reqwest::Response;
use url::Url;

use crate::client::{TDError, Twelvedata};

type Result<T> = std::result::Result<T, TDError>;

// Holds a reference to the TwelveData client
pub struct MarketData<'a> {
    pub client: &'a Twelvedata,
}

impl<'a> MarketData<'a> {
    pub fn time_series_url(
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

    pub fn eod_url(&self, symbol: &str, date: &str) -> Result<Url> {
        self.client
            .build_url("eod", &[("symbol", symbol), ("date", date)])
    }

    pub async fn get_time_series(
        &self,
        symbol: &str,
        interval: &str,
        start_date: Option<&str>,
        end_date: Option<&str>,
    ) -> Result<Response> {
        let url = self.time_series_url(symbol, interval, start_date, end_date)?;
        self.client.make_request(url).await
    }
}
