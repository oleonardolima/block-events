extern crate env_logger;

mod mempool_space;

use anyhow::anyhow;
use futures_core::Stream;
pub use mempool_space::api::{
    BlockEvent, MempoolSpaceWebSocketMessage, MempoolSpaceWebSocketRequestData,
};
pub use mempool_space::get_default_websocket_address;
use mempool_space::websocket::subscribe_to_blocks;
use url::Url;

pub async fn fetch_data_stream(
    url: &Url,
    data: &MempoolSpaceWebSocketRequestData,
) -> anyhow::Result<impl Stream<Item = BlockEvent>> {
    match data {
        MempoolSpaceWebSocketRequestData::Blocks => subscribe_to_blocks(url).await,
        MempoolSpaceWebSocketRequestData::MempoolBlocks => {
            return Err(anyhow!(
                "currently the mempool-blocks feature is no implemented yet."
            ));
        }
        MempoolSpaceWebSocketRequestData::TrackAddress(_address) => {
            return Err(anyhow!(
                "currently the track-address feature is no implemented yet."
            ));
        }
    }
}
