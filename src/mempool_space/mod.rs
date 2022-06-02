pub mod api;
pub mod websocket;

use anyhow;
use api::{BlockEvent, MempoolSpaceWebSocketRequestData, MempoolSpaceWebSocketRequestMessage};
use bitcoin::Network;
use futures_core::Stream;
use url::Url;

pub async fn subscribe_to_new_blocks(
    network: &Network,
    message: &MempoolSpaceWebSocketRequestMessage,
) -> anyhow::Result<impl Stream<Item = BlockEvent>> {
    let url: Url = url::Url::parse(&build_websocket_address(&network)).unwrap();
    websocket::connect_and_publish_message(url, &message).await
}

pub fn build_websocket_request_message(
    data: &MempoolSpaceWebSocketRequestData,
) -> MempoolSpaceWebSocketRequestMessage {
    let mut message = MempoolSpaceWebSocketRequestMessage {
        action: String::from("want"),
        data: vec![],
    };

    match data {
        MempoolSpaceWebSocketRequestData::Blocks => message.data.push(String::from("blocks")),
        MempoolSpaceWebSocketRequestData::MempoolBlocks => {
            message.data.push(String::from("mempool-blocks"))
        }
        // FIXME: (@leonardo.lima) fix this track-address to use different struct
        MempoolSpaceWebSocketRequestData::TrackAddress(..) => { /* ignore */ }
    }
    message
}

fn build_websocket_address(network: &Network) -> String {
    match network {
        Network::Bitcoin => String::from("wss://mempool.space/api/v1/ws"),
        Network::Regtest => String::from("ws://localhost/api/v1/ws"),
        _ => {
            return format!(
                "wss://mempool.space/{}/api/v1/ws",
                Network::to_string(network)
            )
        }
    }
}
