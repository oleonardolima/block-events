#[derive(serde::Deserialize, Debug)]
pub struct BlockExtended {
  pub id: String, // TODO: (@leonardo.lima) parse this into BlockHash type from rust-bitcoin
  pub height: u32,
  // pub version: String,
  pub timestamp: u32,
  // pub bits: String,
  // pub nonce: String,
  // pub difficulty: String,
  // pub merkle_root: String,
  // pub tx_count: String,
  // pub size: String,
  // pub weight: String,
  pub previousblockhash: String, // TODO: (@leonardo.lima) parse this into BlockHash type from rust-bitcoin
  // pub extras: BlockExtension,
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
  TrackAddress(String), // TODO:(@leonardo.lima) Update it to use bitcoin::Address instead
}
