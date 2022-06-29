mod api;
mod http;
mod websocket;

use std::pin::Pin;
use std::time::Duration;

use api::{BlockEvent, BlockExtended};

use anyhow::{anyhow, Ok};
use async_stream::stream;
use bitcoin::{BlockHash, BlockHeader};
use futures_util::stream::Stream;
use tokio::time::Instant;
use tokio_stream::StreamExt;
use url::Url;

const DEFAULT_CONCURRENT_REQUESTS: u8 = 4;

pub async fn subscribe_to_blocks(
    url: &Url,
    checkpoint: Option<(u32, BlockHash)>,
) -> anyhow::Result<Pin<Box<dyn Stream<Item = BlockExtended>>>> {
    // TODO: (@leonardo.lima) It's needed to infer the tls security from network, or feature ?
    let ws_url = &url::Url::parse(format!("ws://{}/ws", url).as_str()).unwrap();
    let http_url = &url::Url::parse(format!("http://{}", url).as_str()).unwrap();

    match checkpoint {
        Some(checkpoint) => {
            let prev_blocks = fetch_previous_blocks(http_url, checkpoint).await?;
            let new_blocks = websocket::subscribe_to_blocks(ws_url).await?;
            // FIXME: This should filter for duplicated blocks
            Ok(Box::pin(prev_blocks.chain(new_blocks)))
        }
        _ => Ok(Box::pin(websocket::subscribe_to_blocks(ws_url).await?)),
    }
}

// FIXME: this fails when checkpoint is genesis block as it does not have a previousblockhash field
pub async fn fetch_previous_blocks(
    url: &Url,
    checkpoint: (u32, BlockHash),
) -> anyhow::Result<impl Stream<Item = BlockExtended>> {
    let client = http::HttpClient::new(url, DEFAULT_CONCURRENT_REQUESTS);
    let (ckpt_height, ckpt_hash) = checkpoint;

    if ckpt_hash != client._get_block_height(ckpt_height).await? {
        return Err(anyhow!(
            "The checkpoint passed is invalid, it should exist in the blockchain."
        ));
    }

    let mut tip = client._get_height().await?;
    let mut height = ckpt_height;

    let mut interval = Instant::now(); // should try to update the tip every 5 minutes.
    let stream = stream! {
        while height <= tip {
            let hash = client._get_block_height(height).await.unwrap();
            let block = client._get_block(hash).await.unwrap();

            height += 1;

            if interval.elapsed() >= Duration::from_secs(300) {
                interval = Instant::now();
                tip = client._get_height().await.unwrap();
            }
            yield block;
        }
    };
    Ok(stream)
}
