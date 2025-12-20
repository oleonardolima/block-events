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

    // async fetch the data stream through the lib
    let block_events = block_events::subscribe_to_blocks(http_url, ws_base_url, None).await?;

    // consume and execute the code (current matching and printing) in async manner for each new block-event
    pin_mut!(block_events);
    while let Some(block_event) = block_events.next().await {
        println!("{:#?}", block_event);
    }
    Ok(())
}
