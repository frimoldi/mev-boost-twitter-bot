use crate::twitter::BigRewardTweet;
use dotenv::dotenv;
use ethers::{types::U256, utils::format_units};
use redis::Commands;
use std::thread::sleep;
use std::time::Duration;
extern crate redis;

mod ethereum;
mod flashbots_api;
mod twitter;

#[tokio::main]
async fn main() {
    dotenv().ok();

    loop {
        process_slots().await;
        sleep(Duration::from_secs(30));
    }
}

fn fetch_last_processed_slot() -> redis::RedisResult<u32> {
    // connect to redis
    let redis_uri = std::env::var("REDIS_URI").expect("REDIS_URI must be set.");

    let client = redis::Client::open(redis_uri)?;
    let mut con = client.get_connection()?;

    con.get("last_processed_slot")
}

fn set_last_processed_slot(slot: u32) -> redis::RedisResult<u32> {
    let redis_uri = std::env::var("REDIS_URI").expect("REDIS_URI must be set.");
    // connect to redis
    let client = redis::Client::open(redis_uri)?;
    let mut con = client.get_connection()?;

    con.set("last_processed_slot", slot)
}

async fn process_slots() {
    let last_processed_slot = match fetch_last_processed_slot() {
        Ok(result) => result,
        Err(err) => {
            println!("{}", err);
            0
        }
    };

    println!("Processing slots from {last_processed_slot} ");

    let last_processed_slot = process_slots_from(last_processed_slot).await;

    println!("Finished at slot {last_processed_slot}");

    match set_last_processed_slot(last_processed_slot) {
        Ok(_) => {}
        Err(err) => {
            println!("{}", err);
        }
    }
}

async fn process_slots_from(slot_from: u32) -> u32 {
    let mut last_processed_slot = slot_from;

    match flashbots_api::fetch_payloads().await {
        Ok(payloads) => {
            for payload in payloads {
                if let (Ok(slot), Ok(value)) = (
                    payload.slot.parse::<u32>(),
                    U256::from_dec_str(&payload.value),
                ) {
                    let big_reward_min = U256::from_dec_str("100000000000000000").unwrap();
                    if slot > slot_from && value > big_reward_min {
                        if let Ok(Some(block)) = ethereum::get_block(&payload.block_hash).await {
                            if let (Some(block_number), Ok(value)) =
                                (block.number, format_units(value, "ether"))
                            {
                                let tweet = BigRewardTweet {
                                    block_number: block_number.as_u32(),
                                    value,
                                };

                                match twitter::publish_tweet(&tweet).await {
                                    Ok(()) => println!("Tweet sent!"),
                                    Err(_) => println!("Tweet failed"),
                                }
                            }
                        }
                    }

                    last_processed_slot = slot;
                }
            }
        }

        Err(_) => println!("Fetch payloads: Something went wrong :("),
    }

    last_processed_slot
}
