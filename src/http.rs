// Block Events Library
// Written in 2022 by Leonardo Lima <> and Lloyd Fournier <>
//
// This file is licensed under the Apache License, Version 2.0 <LICENSE-APACHE
// or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// You may not use this file except in accordance with one or both of these
// licenses.

//! Http client implementation for mempool.space available endpoints
//! It used `reqwest` async client

use bitcoin::BlockHash;
use reqwest::Client;
use url::Url;

use crate::api::BlockExtended;

/// Generic HttpClient using `reqwest`
/// It has been based on the Esplora client from BDK
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct HttpClient {
    url: String,
    client: Client,
    concurrency: u8,
}

impl HttpClient {
    /// Creates a new HttpClient, for given base url and concurrency
    pub fn new(base_url: &Url, concurrency: u8) -> Self {
        let url = url::Url::parse(format!("http://{}", base_url).as_str()).unwrap();
        HttpClient {
            url: url.to_string(),
            client: Client::new(),
            concurrency,
        }
    }

    /// Get current blockchain block height (the current tip height)
    pub async fn _get_height(&self) -> anyhow::Result<u32> {
        let req = self
            .client
            .get(&format!("{}/blocks/tip/height", self.url))
            .send()
            .await?;

        Ok(req.error_for_status()?.text().await?.parse()?)
    }

    /// Get [`BlockHash`] from mempool.space, for given block height
    pub async fn _get_block_height(&self, height: u32) -> anyhow::Result<BlockHash> {
        let req = self
            .client
            .get(&format!("{}/block-height/{}", self.url, height))
            .send()
            .await?;

        Ok(req.error_for_status()?.text().await?.parse()?)
    }

    /// Get [`BlockExtended`] from mempool.space, by [`BlockHash`]
    pub async fn _get_block(&self, block_hash: BlockHash) -> anyhow::Result<BlockExtended> {
        let req = self
            .client
            .get(&format!("{}/block/{}", self.url, block_hash))
            .send()
            .await?;

        Ok(serde_json::from_str(req.error_for_status()?.text().await?.as_str()).unwrap())
    }
}
