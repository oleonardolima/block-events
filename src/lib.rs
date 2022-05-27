extern crate env_logger;

mod mempool_space;

pub use mempool_space::api::{MempoolSpaceWebSocketMessage, MempoolSpaceWebSocketRequestData};
use anyhow::Ok;
use bitcoin::Network;
use futures_util::StreamExt;
use mempool_space::{subscribe_to_new_blocks, build_websocket_request_message};

use futures_util::pin_mut;

pub async fn fetch_data_stream(network: &Network, data: &MempoolSpaceWebSocketRequestData) -> anyhow::Result<()> {
    env_logger::init();

    match data {
        MempoolSpaceWebSocketRequestData::Blocks => {
            let message = build_websocket_request_message(&data);
            let block_stream = subscribe_to_new_blocks(&network, &message).await?;
            pin_mut!(block_stream);

            while let Some(block) = block_stream.next().await {
                println!("received following new block: {:#?}", block);
            };
        },
        MempoolSpaceWebSocketRequestData::MempoolBlocks => { eprintln!("currently the mempool-blocks feature is no implemented yet.") },
        MempoolSpaceWebSocketRequestData::TrackAddress(_address) => { eprintln!("currently the track-address feature is no implemented yet.") },
    }

    Ok(())
}
