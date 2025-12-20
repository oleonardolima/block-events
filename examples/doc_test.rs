use anyhow::{self, Ok};
use futures::{pin_mut, StreamExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    // for mempool.space testnet network
    let http_base_url = "http://mempool.space/testnet/api/";
    let ws_base_url = "wss://mempool.space/testnet";

    // no checkpoint for this example, but you could use the following one to test it by yourself (in mainnet).
    // checkpoint for first BDK Taproot transaction on mainnet (base_url update needed)
    // let checkpoint = (709635, bitcoin::BlockHash::from("00000000000000000001f9ee4f69cbc75ce61db5178175c2ad021fe1df5bad8f"));
    let checkpoint = None;

    // async fetch the block-events stream through the lib
    let block_events =
        block_events::subscribe_to_block_headers(http_base_url, ws_base_url, checkpoint).await?;

    // consume and execute your code (current only matching and printing) in async manner for each new block-event
    pin_mut!(block_events);
    while let Some(block_event) = block_events.next().await {
        match block_event? {
            block_events::api::BlockEvent::Connected(block_header) => {
                println!("[connected][block_header] {:#?}", block_header);
            }
            block_events::api::BlockEvent::Disconnected((height, block_hash)) => {
                println!(
                    "[disconnected][height: {:#?}][block_hash: {:#?}]",
                    height, block_hash
                );
            }
        }
    }
    Ok(())
}
