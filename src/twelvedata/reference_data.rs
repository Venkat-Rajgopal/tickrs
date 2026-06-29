use url::Url;

use crate::twelvedata::client::{Result, Twelvedata};
use crate::twelvedata::data::SymbolSearchResponse;

pub struct ReferenceData<'a> {
    pub client: &'a Twelvedata,
}
impl<'a> ReferenceData<'a> {
    /// Example: /symbol_search?symbol=AAPL

    pub fn build_symbol_search_url(&self, symbol: &str) -> Result<Url> {
        self.client
            .build_url("symbol_search", &[("symbol", symbol)])
    }

    pub async fn get_symbol_search(&self, symbol: &str) -> Result<SymbolSearchResponse> {
        let url = self.build_symbol_search_url(symbol)?;
        let response = self.client.make_request(url).await?;
        let data = response.json::<SymbolSearchResponse>().await?;
        Ok(data)
    }
}

#[cfg(test)]
mod tests {
    use crate::twelvedata::{
        client::{Result, Twelvedata},
        reference_data::ReferenceData,
    };

    #[test]
    fn test_get_symbol_search_url() -> Result<()> {
        let client = Twelvedata::test_client("https://api.twelvedata.com")?;
        let reference_data = ReferenceData { client: &client };

        let result = reference_data.build_symbol_search_url("RELIANCE")?;

        // Check the base URL and endpoint
        assert_eq!(result.scheme(), "https");
        assert_eq!(result.host_str(), Some("api.twelvedata.com"));
        assert_eq!(result.path(), "/symbol_search");

        // Check query parameters
        let query_params: std::collections::HashMap<_, _> = result.query_pairs().collect();
        assert_eq!(
            query_params.get("symbol").map(|s| s.as_ref()),
            Some("RELIANCE")
        );
        assert!(query_params.contains_key("apikey"));

        Ok(())
    }
}
