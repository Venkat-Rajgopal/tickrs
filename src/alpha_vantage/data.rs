use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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

#[derive(Debug, Serialize, Deserialize)]
pub struct TimeSeriesResponse {
    #[serde(rename = "Meta Data")]
    pub meta_data: MetaData,
    #[serde(rename = "Weekly Time Series")]
    pub weekly_time_series: HashMap<String, TimeSeriesData>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MetaData {
    #[serde(rename = "1. Information")]
    pub information: String,
    #[serde(rename = "2. Symbol")]
    pub symbol: String,
    #[serde(rename = "3. Last Refreshed")]
    pub last_refreshed: String,
    #[serde(rename = "4. Time Zone")]
    pub time_zone: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimeSeriesData {
    #[serde(rename = "4. close")]
    pub close: String,
}
