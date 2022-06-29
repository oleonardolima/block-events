use bitcoin::BlockHash;
use reqwest::Client;
use url::Url;

use crate::api::BlockExtended;

// #[derive(Debug)]
pub struct HttpClient {
    url: String,
    client: Client,
    concurrency: u8,
}

impl HttpClient {
    pub fn new(base_url: &Url, concurrency: u8) -> Self {
        HttpClient {
            url: base_url.to_string(),
            client: Client::new(),
            concurrency: concurrency,
        }
    }

    pub async fn _get_height(&self) -> anyhow::Result<u32> {
        let req = self
            .client
            .get(&format!("{}/blocks/tip/height", self.url))
            .send()
            .await?;

        Ok(req.error_for_status()?.text().await?.parse()?)
    }

    pub async fn _get_block_height(&self, height: u32) -> anyhow::Result<BlockHash> {
        let req = self
            .client
            .get(&format!("{}/block-height/{}", self.url, height))
            .send()
            .await?;

        Ok(req.error_for_status()?.text().await?.parse()?)
    }

    pub async fn _get_block(&self, block_hash: BlockHash) -> anyhow::Result<BlockExtended> {
        let req = self
            .client
            .get(&format!("{}/block/{}", self.url, block_hash))
            .send()
            .await?;

        Ok(serde_json::from_str(req.error_for_status()?.text().await?.as_str()).unwrap())
    }
}
