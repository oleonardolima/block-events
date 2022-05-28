use bitcoin::{Address, BlockHash};

#[derive(serde::Deserialize, Clone, Debug)]
pub struct BlockExtended {
  pub height: u32,
  pub timestamp: u32,
  pub id: BlockHash,
  pub previousblockhash: BlockHash,
}

#[derive(serde::Deserialize, Debug)]
pub struct MempoolSpaceWebSocketMessage {
  pub block: BlockExtended,
}

#[derive(serde::Serialize, Debug)]
pub struct MempoolSpaceWebSocketRequestMessage {
  pub action: String,
  pub data: Vec<String>,
}

pub enum MempoolSpaceWebSocketRequestData {
  Blocks,
  MempoolBlocks,
  TrackAddress(Address), // TODO:(@leonardo.lima) Update it to use bitcoin::Address instead
}
