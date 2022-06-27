pub mod api;
pub mod http;
pub mod websocket;

use std::pin::Pin;

use api::BlockEvent;

use anyhow::{anyhow, Ok};
use async_stream::{stream, try_stream};
use bitcoin::Block;
use futures_util::{pin_mut, stream::Stream};
use tokio_stream::StreamExt;
use url::Url;

pub async fn subscribe_to_blocks(
    url: &Url,
    height: Option<u32>,
) -> (
    Option<anyhow::Result<impl Stream<Item = BlockEvent>>>,
    anyhow::Result<impl Stream<Item = BlockEvent>>,
) {
    log::debug!("[height.is_none] {:?}", height.is_none());

    // TODO: (@leonardo.lima) It's needed to infer the tls security from network, or feature ?
    let ws_url = &url::Url::parse(format!("ws://{}/ws", url).as_str()).unwrap();
    let http_url = &url::Url::parse(format!("http://{}", url).as_str()).unwrap();

    match height {
        Some(height) => {
            // let prev_blocks = fetch_previous_blocks(http_url, height).await?;
            // let new_blocks = websocket::subscribe_to_blocks(ws_url).await?;

            // pin_mut!(prev_blocks);
            // pin_mut!(new_blocks);

            // let stream = stream! {
            //     while let Some(prev_block) = prev_blocks.next().await {
            //         yield prev_block.clone();
            //     }

            //     while let Some(new_block) = new_blocks.next().await {
            //         yield new_block.clone();
            //     }
            // };
            // Ok(stream)

            let prev_blocks = fetch_previous_blocks(http_url, height).await;
            let new_blocks = websocket::subscribe_to_blocks(ws_url).await;
            return (Some(prev_blocks), new_blocks);
        }
        _ => return (None, websocket::subscribe_to_blocks(ws_url).await),
    }
}

pub async fn fetch_previous_blocks(
    url: &Url,
    mut height: u32,
) -> anyhow::Result<impl Stream<Item = BlockEvent>> {
    // TODO: (@leonardo.lima) Move the concurrency for an environment variable
    let http_client = http::HttpClient::new(url, 4);
    let mut curr_tip = http_client._get_height().await.unwrap();

    log::debug!("[curr_tip {}]", &curr_tip);
    log::debug!("[height {}]", &height);

    let stream = stream! {
        while height <= curr_tip {
            let block_hash = http_client._get_block_height(height).await.unwrap();
            let block = http_client._get_block(block_hash).await.unwrap();

            log::debug!("[curr_tip {}]", &curr_tip);
            log::debug!("[height {}]", &height);
            log::debug!("[block {:#?}]", &block);

            // TODO: (@leonardo.lima) The update in current tip should have some time in between, and not at every iteration
            curr_tip = http_client._get_height().await.unwrap();
            height += 1;
            yield BlockEvent::Connected(block);
        }
    };
    Ok(stream)
}
