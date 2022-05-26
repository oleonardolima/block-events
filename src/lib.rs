extern crate env_logger;

mod mempool_space;

pub use mempool_space::api::MempoolSpaceWebSocketMessage;
use bitcoin::Network;
use mempool_space::fetch_new_blocks;

pub async fn fetch_data(network: Network, _data: Vec<String>) {
    env_logger::init();
    // TODO: (@leonardo.lima) The data needs to be parsed in order to know which fn to use from mempool.space module

    fetch_new_blocks(network).await;
}
