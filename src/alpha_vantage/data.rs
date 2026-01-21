use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
