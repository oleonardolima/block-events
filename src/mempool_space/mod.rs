pub mod api;
pub mod websocket;

use bitcoin::Network;

pub fn get_default_websocket_address(network: &Network) -> String {
    match network {
        Network::Bitcoin => String::from("wss://mempool.space/api/v1/ws"),
        Network::Regtest => String::from("ws://localhost:8999/api/v1/ws"),
        _ => {
            return format!(
                "wss://mempool.space/{}/api/v1/ws",
                Network::to_string(network)
            )
        }
    }
}
