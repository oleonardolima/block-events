extern crate env_logger;

mod mempool_space;

pub use mempool_space::{subscribe_to_new_blocks, build_websocket_request_message};
pub use mempool_space::api::{MempoolSpaceWebSocketMessage, MempoolSpaceWebSocketRequestData, BlockEvent};
use anyhow::{anyhow};
use bitcoin::Network;
use futures_core::Stream;

pub async fn fetch_data_stream(network: &Network, data: &MempoolSpaceWebSocketRequestData) -> anyhow::Result<impl Stream<Item = BlockEvent>> {
    env_logger::init();

    match data {
        MempoolSpaceWebSocketRequestData::Blocks => {
            let message = build_websocket_request_message(&data);
            subscribe_to_new_blocks(&network, &message).await
        },
        MempoolSpaceWebSocketRequestData::MempoolBlocks => {
            return Err(anyhow!("currently the mempool-blocks feature is no implemented yet."));
        },
        MempoolSpaceWebSocketRequestData::TrackAddress(_address) => {
            return Err(anyhow!("currently the track-address feature is no implemented yet."));
        },
    }
}
