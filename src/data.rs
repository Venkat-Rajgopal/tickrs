use polars::prelude::*;
use serde::{Deserialize, Serialize};
// use std::io::Cursor;

/// Trait for converting types to Polars DataFrame
pub trait ToDataFrame {
    fn to_dataframe(&self) -> PolarsResult<DataFrame>;
}

/// Generic conversion for any Serialize type
/// Uses JSON as intermediate format
// pub fn to_dataframe_generic<T: Serialize>(data: &T) -> PolarsResult<DataFrame> {
//     let json = serde_json::to_string(data).map_err(|e| {
//         PolarsError::ComputeError(format!("JSON serialization failed: {}", e).into())
//     })?;

//     JsonReader::new(Cursor::new(json.as_bytes())).finish()
// }

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


