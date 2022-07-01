mod api;
mod http;
mod websocket;

use std::time::Duration;
use std::{collections::HashMap, collections::VecDeque, pin::Pin};

use api::{BlockEvent, BlockExtended};

use anyhow::{anyhow, Ok};
use async_stream::stream;
use bitcoin::{BlockHash, BlockHeader};
use futures_util::stream::Stream;
use http::HttpClient;
use tokio::time::Instant;
use tokio_stream::StreamExt;
use url::Url;

const DEFAULT_CONCURRENT_REQUESTS: u8 = 4;

#[derive(Debug, Clone)]
struct Cache {
    tip: BlockHash,
    active_headers: HashMap<BlockHash, BlockExtended>,
    stale_headers: HashMap<BlockHash, BlockExtended>,
}

// TODO: (@leonardo.lima) The `BlockExtended` should be converted/translated to `BlockHeader`
pub async fn subscribe_to_blocks(
    url: &Url,
    checkpoint: Option<(u32, BlockHash)>,
) -> anyhow::Result<Pin<Box<dyn Stream<Item = BlockEvent>>>> {
    // TODO: (@leonardo.lima) It's needed to infer the tls security from network, or feature ?
    let ws_url = url::Url::parse(format!("ws://{}/ws", url).as_str()).unwrap();
    let http_url = url::Url::parse(format!("http://{}", url).as_str()).unwrap();

    let client = http::HttpClient::new(&http_url, DEFAULT_CONCURRENT_REQUESTS);
    let chain_height = client._get_height().await.unwrap();
    let chain_tip = client._get_block_height(chain_height).await.unwrap();
    let cache = Cache {
        tip: chain_tip,
        active_headers: HashMap::new(),
        stale_headers: HashMap::new(),
    };

    match checkpoint {
        Some(checkpoint) => {
            let prev_blocks = fetch_previous_blocks(&http_url, checkpoint).await?;
            let new_blocks = websocket::subscribe_to_blocks(&ws_url).await?;
            // FIXME: This should filter for duplicated blocks
            let events =
                process_candidates(client, cache, Box::pin(prev_blocks.chain(new_blocks))).await?;
            Ok(Box::pin(events))
        }
        _ => {
            let candidates = websocket::subscribe_to_blocks(&ws_url).await?;
            let events = process_candidates(client, cache, Box::pin(candidates)).await?;
            Ok(Box::pin(events))
        }
    }
}

async fn process_candidates(
    client: HttpClient,
    mut cache: Cache,
    mut candidates: Pin<Box<dyn Stream<Item = BlockExtended>>>,
) -> anyhow::Result<impl Stream<Item = BlockEvent>> {
    let stream = stream! {
        while let Some(candidate) = candidates.next().await {
            // TODO: (@leonardo.lima) It should check and validate for valid BlockHeaders

            // validate if its a new valid tip
            if cache.tip == candidate.prev_blockhash {
                cache.tip = candidate.id;
                cache.active_headers.insert(candidate.id, candidate);
                yield BlockEvent::Connected(BlockHeader::from(candidate.clone()));
                continue
            }

            // find common ancestor for current active chain and the forked chain
            // fetches forked chain candidates and store in cache
            let mut common_ancestor = candidate.clone();
            let mut fork_branch: VecDeque<BlockExtended> = VecDeque::new();
            while !cache.active_headers.contains_key(&common_ancestor.id) {
                log::debug!("{:?}", common_ancestor);
                fork_branch.push_back(common_ancestor);
                common_ancestor = client._get_block(common_ancestor.prev_blockhash).await.unwrap(); // TODO: (@leonardo.lima) improve error handling here
            }

            // rollback current active chain, moving blocks to staled field
            // yields BlockEvent::Disconnected((u32, BlockHash))
            while common_ancestor.id != cache.tip {
                let (stale_hash, stale_header) = cache.active_headers.remove_entry(&cache.tip).unwrap();
                cache.stale_headers.insert(stale_hash, stale_header);
                cache.tip = common_ancestor.id;
                yield BlockEvent::Disconnected((stale_header.height, stale_hash));
            }

            // iterate over forked chain candidates
            // update [`Cache`] active_headers field with candidates
            // yields BlockEvent::Connected(candidate)
            for fork_candidate in fork_branch.iter() {
                cache.active_headers.insert(fork_candidate.id, fork_candidate.clone());
                cache.tip = fork_candidate.id;
                yield BlockEvent::Connected(BlockHeader::from(fork_candidate.clone()));
            }
            // yield BlockEvent::Connected(BlockHeader::from(candidate.clone()));
        }
    };
    Ok(stream)
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
