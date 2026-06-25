use polars::prelude::*;
use serde::{Deserialize, Deserializer, Serialize};
use std::collections::HashMap;

pub trait ToDataFrame {
    fn to_dataframe(&self) -> PolarsResult<DataFrame>;
}

/// Response from the SYMBOL_SEARCH endpoint
#[derive(Debug, Serialize, Deserialize)]
pub struct SymbolSearchResponse {
    #[serde(rename = "bestMatches")]
    pub best_matches: Vec<SymbolMatch>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SymbolMatch {
    #[serde(rename = "1. symbol")]
    pub symbol: String,
    #[serde(rename = "2. name")]
    pub name: String,
    #[serde(rename = "3. type")]
    pub instrument_type: String,
    #[serde(rename = "4. region")]
    pub region: String,
    #[serde(rename = "5. marketOpen")]
    pub market_open: String,
    #[serde(rename = "6. marketClose")]
    pub market_close: String,
    #[serde(rename = "7. timezone")]
    pub timezone: String,
    #[serde(rename = "8. currency")]
    pub currency: String,
    #[serde(rename = "9. matchScore")]
    pub match_score: String,
}

// ===== TIME SERIES RESPONSE STRUCTURES =====

/// AV returns a different outer key per interval ("Weekly Time Series",
/// "Time Series (Daily)", "Time Series (1min)", etc.). The custom Deserialize
/// impl skips "Meta Data" and takes whatever remaining key holds the series.
#[derive(Debug, Serialize)]
pub struct TimeSeriesResponse {
    pub time_series: HashMap<String, TimeSeriesData>,
}

impl<'de> Deserialize<'de> for TimeSeriesResponse {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::Error;
        let mut map = serde_json::Map::<String, serde_json::Value>::deserialize(deserializer)?;

        map.remove("Meta Data");

        let (_, series_value) = map
            .into_iter()
            .next()
            .ok_or_else(|| D::Error::custom("no time series key found in response"))?;

        let time_series: HashMap<String, TimeSeriesData> =
            serde_json::from_value(series_value).map_err(D::Error::custom)?;

        Ok(TimeSeriesResponse { time_series })
    }
}

impl ToDataFrame for TimeSeriesResponse {
    fn to_dataframe(&self) -> PolarsResult<DataFrame> {
        if self.time_series.is_empty() {
            return Err(PolarsError::ComputeError("No time series data".into()));
        }

        let mut dates: Vec<String> = self.time_series.keys().cloned().collect();
        dates.sort_by(|a, b| b.cmp(a)); // newest first

        let mut opens = Vec::new();
        let mut highs = Vec::new();
        let mut lows = Vec::new();
        let mut closes = Vec::new();
        let mut volumes = Vec::new();

        for date in &dates {
            let d = &self.time_series[date];
            opens.push(d.open.parse::<f64>().unwrap_or(0.0));
            highs.push(d.high.parse::<f64>().unwrap_or(0.0));
            lows.push(d.low.parse::<f64>().unwrap_or(0.0));
            closes.push(d.close.parse::<f64>().unwrap_or(0.0));
            volumes.push(d.volume.parse::<i64>().unwrap_or(0));
        }

        DataFrame::new(vec![
            Series::new("date".into(), dates),
            Series::new("open".into(), opens),
            Series::new("high".into(), highs),
            Series::new("low".into(), lows),
            Series::new("close".into(), closes),
            Series::new("volume".into(), volumes),
        ])
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimeSeriesData {
    #[serde(rename = "1. open")]
    pub open: String,
    #[serde(rename = "2. high")]
    pub high: String,
    #[serde(rename = "3. low")]
    pub low: String,
    #[serde(rename = "4. close")]
    pub close: String,
    #[serde(rename = "5. volume")]
    pub volume: String,
}
