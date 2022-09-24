use serde::Deserialize;
use std::thread::sleep;
use std::time::Duration;

extern crate redis;
use redis::Commands;

const FLASHBOTS_API_URL: &str =
    "https://boost-relay.flashbots.net/relay/v1/data/bidtraces/proposer_payload_delivered";

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
    loop {
        process_slots().await;
        sleep(Duration::from_secs(30));
    }
}

fn fetch_last_processed_slot() -> redis::RedisResult<u32> {
    // connect to redis
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_connection()?;

    con.get("last_processed_slot")
}

fn set_last_processed_slot(slot: u32) -> redis::RedisResult<u32> {
    // connect to redis
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_connection()?;

    con.set("last_processed_slot", slot)
}

async fn process_slots() {
    let last_processed_slot = match fetch_last_processed_slot() {
        Ok(result) => result,
        Err(_) => 0,
    };

    println!("Processing slots from {last_processed_slot} ");

    let last_processed_slot = process_slots_from(last_processed_slot).await;

    println!("Finished at slot {last_processed_slot}");

    let _ = set_last_processed_slot(last_processed_slot);
}

async fn process_slots_from(slot_from: u32) -> u32 {
    let mut last_processed_slot = slot_from;
    let result = get_payloads().await;

    match result {
        Ok(payloads) => {
            for payload in payloads {
                let slot: u32 = match payload.slot.parse() {
                    Ok(v) => {
                        if v > slot_from {
                            v
                        } else {
                            continue;
                        }
                    }
                    Err(_) => continue,
                };
                let value: u128 = match payload.value.parse() {
                    Ok(v) => v,
                    Err(_) => continue,
                };

                if value > REWARD_VALUE_THRESHOLD {
                    println!("Big reward! {value}");
                }

                last_processed_slot = slot;
            }
        }
        Err(_) => println!("Something went wrong :("),
    }

    last_processed_slot
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
