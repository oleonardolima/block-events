pub mod api;
pub mod websocket;

use api::{MempoolSpaceWebSocketRequestMessage};
use bitcoin::Network;
use url::Url;

// TODO: (@leonardo.lima)
// pub async fn fetch_new_mempool_blocks(network: Network);
// pub async fn track_tx(network: Network, tx: String);
// pub async fn track_address(network: Network, address: String);

pub async fn fetch_new_blocks(network: Network) {
  let url: Url = url::Url::parse(&build_websocket_address(network)).unwrap();

  let message = MempoolSpaceWebSocketRequestMessage {
    action: String::from("want"),
    data: vec![String::from("block")],
  };

  websocket::publish_message(url, message).await.unwrap();
}

// TODO: (@leonardo.lima) refactor this fn to use constants for base url and formatting
fn build_websocket_address(network: Network) -> String {
  match network {
    Network::Bitcoin => String::from("wss://mempool.space/api/v1/ws"),
    Network::Testnet => String::from("wss://mempool.space/testnet/api/v1/ws"),
    Network::Signet => String::from("wss://mempool.space/signet/api/v1/ws"),
    Network::Regtest => String::from("ws://localhost/api/v1/ws"),
  }
}
