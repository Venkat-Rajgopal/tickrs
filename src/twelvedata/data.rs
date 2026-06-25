use polars::prelude::*;
use serde::{Deserialize, Serialize};

/// Trait for converting types to Polars DataFrame
pub trait ToDataFrame {
    fn to_dataframe(&self) -> PolarsResult<DataFrame>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SymbolSearchResponse {
    pub data: Vec<InstrumentData>,
    pub status: String,
}

impl ToDataFrame for SymbolSearchResponse {
    fn to_dataframe(&self) -> PolarsResult<DataFrame> {
        // Option 1: Manual (more control, better performance)
        let mut symbols = Vec::new();
        let mut instrument_names = Vec::new();
        let mut exchanges = Vec::new();
        let mut instrument_types = Vec::new();
        let mut countries = Vec::new();
        let mut currencies = Vec::new();

        for d in &self.data {
            symbols.push(d.symbol.clone());
            instrument_names.push(d.instrument_name.clone());
            exchanges.push(d.exchange.clone());
            instrument_types.push(d.instrument_type.clone());
            countries.push(d.country.clone());
            currencies.push(d.currency.clone());
        }

        DataFrame::new(vec![
            Series::new("symbol".into(), symbols),
            Series::new("instrument_name".into(), instrument_names),
            Series::new("exchange".into(), exchanges),
            Series::new("instrument_type".into(), instrument_types),
            Series::new("country".into(), countries),
            Series::new("currency".into(), currencies),
        ])

        // Option 2: Generic (less code, slight overhead)
        // to_dataframe_generic(&self.data)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstrumentData {
    pub symbol: String,
    pub instrument_name: String,
    pub exchange: String,
    pub instrument_type: String,
    pub country: String,
    pub currency: String,
}

// ===== TIME SERIES RESPONSE STRUCTURES =====

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeSeriesResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<TimeSeriesMeta>,
    #[serde(default)]
    pub values: Vec<TimeSeriesValue>,
    pub status: String,

    // Handle error cases
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeSeriesMeta {
    pub symbol: String,
    pub interval: String,
    pub currency: String,
    pub exchange_timezone: String,
    pub exchange: String,
    pub mic_code: String,
    #[serde(rename = "type")]
    pub instrument_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeSeriesValue {
    pub datetime: String,
    pub open: String,
    pub high: String,
    pub low: String,
    pub close: String,
    #[serde(default)]
    pub volume: Option<String>,
}

impl ToDataFrame for TimeSeriesResponse {
    fn to_dataframe(&self) -> PolarsResult<DataFrame> {
        if self.values.is_empty() {
            return Err(PolarsError::ComputeError(
                format!(
                    "No data available. Status: {}. Message: {:?}",
                    self.status, self.message
                )
                .into(),
            ));
        }

        let mut datetimes = Vec::new();
        let mut opens = Vec::new();
        let mut highs = Vec::new();
        let mut lows = Vec::new();
        let mut closes = Vec::new();
        let mut volumes = Vec::new();

        for value in &self.values {
            datetimes.push(value.datetime.clone());

            // Parse string values to f64
            opens.push(value.open.parse::<f64>().unwrap_or(0.0));
            highs.push(value.high.parse::<f64>().unwrap_or(0.0));
            lows.push(value.low.parse::<f64>().unwrap_or(0.0));
            closes.push(value.close.parse::<f64>().unwrap_or(0.0));

            // Handle optional volume
            volumes.push(value.volume.as_ref().and_then(|v| v.parse::<i64>().ok()));
        }

        DataFrame::new(vec![
            Series::new("datetime".into(), datetimes),
            Series::new("open".into(), opens),
            Series::new("high".into(), highs),
            Series::new("low".into(), lows),
            Series::new("close".into(), closes),
            Series::new("volume".into(), volumes),
        ])
    }
}

// ===== EOD PRICE RESPONSE STRUCTURE =====

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EodResponse {
    pub symbol: String,
    pub exchange: String,
    pub mic_code: String,
    pub currency: String,
    pub datetime: String,
    pub close: String,
}

impl ToDataFrame for EodResponse {
    fn to_dataframe(&self) -> PolarsResult<DataFrame> {
        DataFrame::new(vec![
            Series::new("symbol".into(), vec![self.symbol.clone()]),
            Series::new("exchange".into(), vec![self.exchange.clone()]),
            Series::new("mic_code".into(), vec![self.mic_code.clone()]),
            Series::new("currency".into(), vec![self.currency.clone()]),
            Series::new("datetime".into(), vec![self.datetime.clone()]),
            Series::new(
                "close".into(),
                vec![self.close.parse::<f64>().unwrap_or(0.0)],
            ),
        ])
    }
}

// ===== QUOTE RESPONSE STRUCTURE =====

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuoteResponse {
    pub symbol: String,
    pub name: String,
    pub exchange: String,
    pub mic_code: String,
    pub currency: String,
    pub datetime: String,
    pub timestamp: i64,
    pub open: String,
    pub high: String,
    pub low: String,
    pub close: String,
    pub volume: String,
    pub previous_close: String,
    pub change: String,
    pub percent_change: String,
    #[serde(default)]
    pub average_volume: Option<String>,
    #[serde(default)]
    pub fifty_two_week: Option<FiftyTwoWeek>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FiftyTwoWeek {
    pub low: String,
    pub high: String,
    pub low_change: String,
    pub high_change: String,
    pub low_change_percent: String,
    pub high_change_percent: String,
    pub range: String,
}

impl ToDataFrame for QuoteResponse {
    fn to_dataframe(&self) -> PolarsResult<DataFrame> {
        DataFrame::new(vec![
            Series::new("symbol".into(), vec![self.symbol.clone()]),
            Series::new("name".into(), vec![self.name.clone()]),
            Series::new("exchange".into(), vec![self.exchange.clone()]),
            Series::new("datetime".into(), vec![self.datetime.clone()]),
            Series::new("open".into(), vec![self.open.parse::<f64>().unwrap_or(0.0)]),
            Series::new("high".into(), vec![self.high.parse::<f64>().unwrap_or(0.0)]),
            Series::new("low".into(), vec![self.low.parse::<f64>().unwrap_or(0.0)]),
            Series::new(
                "close".into(),
                vec![self.close.parse::<f64>().unwrap_or(0.0)],
            ),
            Series::new(
                "volume".into(),
                vec![self.volume.parse::<i64>().unwrap_or(0)],
            ),
            Series::new(
                "previous_close".into(),
                vec![self.previous_close.parse::<f64>().unwrap_or(0.0)],
            ),
            Series::new(
                "change".into(),
                vec![self.change.parse::<f64>().unwrap_or(0.0)],
            ),
            Series::new(
                "percent_change".into(),
                vec![self.percent_change.parse::<f64>().unwrap_or(0.0)],
            ),
        ])
    }
}
