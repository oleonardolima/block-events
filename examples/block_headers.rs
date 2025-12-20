// use std::str::FromStr;

use anyhow::Ok;
// use bitcoin::BlockHash;
use futures::{pin_mut, StreamExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    // http base url
    let http_url = "http://localhost:8999/api/v1";
    // websocket base url
    let ws_base_url = "ws://localhost:8999";

    let checkpoint = None;
    // let checkpoint = Some((
    //     0,
    //     BlockHash::from_str("0f9188f13cb7b2c71f2a335e3a4fc328bf5beb436012afca590b1a11466e2206")
    //         .unwrap(),
    // ));

    // async fetch the data stream through the lib
    let block_events =
        block_events::subscribe_to_block_headers(http_url, ws_base_url, checkpoint).await?;

    // consume and execute the code (current matching and printing) in async manner for each new block-event
    pin_mut!(block_events);
    while let Some(block_event) = block_events.next().await {
        println!("{:#?}", block_event);
    }
    Ok(())
}
