use reqwest;
use serde::Deserialize;

const FLASHBOTS_API_URL: &str =
    "https://boost-relay.flashbots.net/relay/v1/data/bidtraces/proposer_payload_delivered";

#[derive(Deserialize, Debug)]
pub struct Payload {
    pub slot: String,
    pub block_hash: String,
    pub value: String,
}

pub async fn fetch_payloads() -> Result<Vec<Payload>, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let mut payloads = client
        .get(FLASHBOTS_API_URL)
        .send()
        .await?
        .json::<Vec<Payload>>()
        .await?;

    payloads.reverse();

    Ok(payloads)
}
