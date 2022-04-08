use anyhow::Result;
use serde::Deserialize;

pub const API_BASE: &str = "https://api-futures.kucoin.com";

#[derive(Default)]
pub struct Client {}

impl Client {
    pub async fn get_open_contract_list(&self) -> Result<Vec<ActiveContract>> {
        let endpoint = "/api/v1/contracts/active";
        let response: ActiveContractResponse = reqwest::Client::new()
            .get(format!("{}{}", API_BASE, endpoint))
            .send()
            .await?
            .json()
            .await?;
        Ok(response.data)
    }
}

#[derive(Deserialize, Debug)]
pub struct ActiveContract {
    pub symbol: String,
    #[serde(rename = "rootSymbol")]
    pub root_symbol: String,
    #[serde(rename = "baseCurrency")]
    pub base_currency: String,
    #[serde(rename = "quoteCurrency")]
    pub quote_currency: String,
}

#[derive(Deserialize, Debug)]
pub struct ActiveContractResponse {
    pub code: String,
    pub data: Vec<ActiveContract>,
}
