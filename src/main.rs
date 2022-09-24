use serde::Deserialize;

const FLASHBOTS_API_URL: &str =
    "https://boost-relay.flashbots.net/relay/v1/data/bidtraces/proposer_payload_delivered?limit=5";

const REWARD_VALUE_THRESHOLD: u128 = 100000000000000000;

#[derive(Deserialize, Debug)]
struct Payload {
    slot: String,
    parent_hash: String,
    builder_pubkey: String,
    proposer_pubkey: String,
    proposer_fee_recipient: String,
    gas_used: String,
    gas_limit: String,
    value: String,
}

#[tokio::main]
async fn main() {
    let result = get_payloads().await;

    match result {
        Ok(payloads) => {
            for payload in payloads {
                let value: u128 = match payload.value.parse() {
                    Ok(v) => v,
                    Err(_) => continue,
                };

                if value > REWARD_VALUE_THRESHOLD {
                    println!("Big reward! {value}");
                }
            }
        }
        Err(_) => println!("Something went wrong :("),
    }
}

async fn get_payloads() -> Result<Vec<Payload>, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let body = client
        .get(FLASHBOTS_API_URL)
        .send()
        .await?
        .json::<Vec<Payload>>()
        .await?;

    Ok(body)
}
