pub mod api;
pub mod http;
pub mod websocket;

use api::BlockEvent;

use anyhow::{anyhow, Ok};
use async_stream::stream;
use futures_util::{stream::Stream, StreamExt};
use url::Url;

pub async fn subscribe_to_blocks(
    url: &Url,
    height: Option<u32>,
) -> anyhow::Result<impl Stream<Item = BlockEvent>> {
    log::debug!("{}", height.is_none());
    if !height.is_none() {
        let http_url = &url::Url::parse(format!("http://{}", url).as_str()).unwrap();
        // TODO: (@leonardo.lima) Move the concurrency for an environment variable
        let http_client = http::HttpClient::new(http_url, 4);
        let mut curr_tip = http_client._get_height().await.unwrap();
        let mut height = height.unwrap();

        log::debug!("{}", curr_tip);
        log::debug!("{}", height);

        stream! {
            while height <= curr_tip {
                let block_hash = http_client._get_block_height(height).await.unwrap();
                let block = http_client._get_block(block_hash).await.unwrap();

                curr_tip = http_client._get_height().await.unwrap();
                height = block.height;
                log::debug!("{:?}", block);
                yield BlockEvent::Connected(block);
            }
        };
    }

    // TODO: (@leonardo.lima) It's needed to infer the tls security from network, or feature ?
    let ws_url = &url::Url::parse(format!("ws://{}/ws", url).as_str()).unwrap();
    websocket::subscribe_to_blocks(ws_url).await
}

pub async fn fetch_blocks(
    url: &Url,
    height: Option<u32>,
) -> anyhow::Result<impl Stream<Item = BlockEvent>> {
    let http_url = &url::Url::parse(format!("http://{}", url).as_str()).unwrap();
    // TODO: (@leonardo.lima) Move the concurrency for an environment variable
    let http_client = http::HttpClient::new(http_url, 4);
    let mut curr_tip = http_client._get_height().await.unwrap();
    let mut height = height.unwrap();

    let stream = stream! {
        while height <= curr_tip {
            let block_hash = http_client._get_block_height(height).await.unwrap();
            let block = http_client._get_block(block_hash).await.unwrap();

            curr_tip = http_client._get_height().await.unwrap();
            height = block.height;
            yield BlockEvent::Connected(block);
        }
    };
    Ok(stream)
}
