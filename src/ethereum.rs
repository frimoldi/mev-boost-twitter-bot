use std::{error::Error, str::FromStr};

use ethers::{
    self,
    types::{Block, TxHash, H256},
};
use ethers_providers::{self, Http, Middleware, Provider};

pub async fn get_block(block_hash: &str) -> Result<Option<Block<TxHash>>, Box<dyn Error>> {
    let ethereum_provider_url =
        std::env::var("ETHEREUM_PROVIDER_URL").expect("ETHEREUM_PROVIDER_URL must be set.");

    let provider = Provider::<Http>::try_from(ethereum_provider_url)?;

    let block_hash = H256::from_str(block_hash)?;

    match provider.get_block(block_hash).await {
        Ok(block) => Ok(block),
        Err(e) => Err(Box::new(e)),
    }
}
