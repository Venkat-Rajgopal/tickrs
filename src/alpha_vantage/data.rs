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

impl ToDataFrame for SymbolSearchResponse {
    fn to_dataframe(&self) -> PolarsResult<DataFrame> {
        let mut symbols = Vec::new();
        let mut names = Vec::new();
        let mut types = Vec::new();
        let mut regions = Vec::new();
        let mut currencies = Vec::new();
        let mut scores = Vec::new();

        for m in &self.best_matches {
            symbols.push(m.symbol.clone());
            names.push(m.name.clone());
            types.push(m.instrument_type.clone());
            regions.push(m.region.clone());
            currencies.push(m.currency.clone());
            scores.push(m.match_score.parse::<f64>().unwrap_or(0.0));
        }

        DataFrame::new(vec![
            Series::new("symbol".into(), symbols),
            Series::new("name".into(), names),
            Series::new("type".into(), types),
            Series::new("region".into(), regions),
            Series::new("currency".into(), currencies),
            Series::new("match_score".into(), scores),
        ])
    }
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

// ===== GLOBAL QUOTE RESPONSE STRUCTURE =====

#[derive(Debug, Serialize, Deserialize)]
pub struct QuoteResponse {
    #[serde(rename = "Global Quote")]
    pub global_quote: GlobalQuote,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GlobalQuote {
    #[serde(rename = "01. symbol")]
    pub symbol: String,
    #[serde(rename = "02. open")]
    pub open: String,
    #[serde(rename = "03. high")]
    pub high: String,
    #[serde(rename = "04. low")]
    pub low: String,
    #[serde(rename = "05. price")]
    pub price: String,
    #[serde(rename = "06. volume")]
    pub volume: String,
    #[serde(rename = "07. latest trading day")]
    pub latest_trading_day: String,
    #[serde(rename = "08. previous close")]
    pub previous_close: String,
    #[serde(rename = "09. change")]
    pub change: String,
    #[serde(rename = "10. change percent")]
    pub change_percent: String,
}

impl ToDataFrame for QuoteResponse {
    fn to_dataframe(&self) -> PolarsResult<DataFrame> {
        let q = &self.global_quote;
        let change_pct = q
            .change_percent
            .trim_end_matches('%')
            .parse::<f64>()
            .unwrap_or(0.0);
        DataFrame::new(vec![
            Series::new("symbol".into(), vec![q.symbol.clone()]),
            Series::new(
                "latest_trading_day".into(),
                vec![q.latest_trading_day.clone()],
            ),
            Series::new("open".into(), vec![q.open.parse::<f64>().unwrap_or(0.0)]),
            Series::new("high".into(), vec![q.high.parse::<f64>().unwrap_or(0.0)]),
            Series::new("low".into(), vec![q.low.parse::<f64>().unwrap_or(0.0)]),
            Series::new("price".into(), vec![q.price.parse::<f64>().unwrap_or(0.0)]),
            Series::new("volume".into(), vec![q.volume.parse::<i64>().unwrap_or(0)]),
            Series::new(
                "previous_close".into(),
                vec![q.previous_close.parse::<f64>().unwrap_or(0.0)],
            ),
            Series::new(
                "change".into(),
                vec![q.change.parse::<f64>().unwrap_or(0.0)],
            ),
            Series::new("change_percent".into(), vec![change_pct]),
        ])
    }
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
