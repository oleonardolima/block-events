extern crate env_logger;

mod mempool_space;

pub use mempool_space::api::MempoolSpaceWebSocketMessage;
use anyhow::Ok;
use bitcoin::Network;
use futures_util::StreamExt;
use mempool_space::fetch_new_blocks;

use futures_util::pin_mut;

pub async fn fetch_data(network: Network, _data: Vec<String>) -> anyhow::Result<()>{
    env_logger::init();
    // TODO: (@leonardo.lima) The data needs to be parsed in order to know which fn to use from mempool.space module

    let block_stream = fetch_new_blocks(network).await?;
    pin_mut!(block_stream);

    while let Some(block) = block_stream.next().await {
        println!("block {:#?}", block);
    };
    Ok(())
}
