use bitcoin::{Address, BlockHash, TxMerkleNode};

#[derive(serde::Deserialize, Clone, Debug, Copy, From)]
pub struct BlockExtended {
    pub id: BlockHash,
    pub height: u32,
    pub version: u32,
    #[serde(alias = "previousblockhash")]
    pub prev_blockhash: BlockHash,
    pub merkle_root: TxMerkleNode,
    #[serde(alias = "timestamp")]
    pub time: u32,
    pub bits: u32,
    pub nonce: u32,
    // add new fields if needed
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
    TrackAddress(Address),
}

#[derive(Debug, Clone, Copy)]
pub enum BlockEvent {
    Connected(BlockExtended),
    Disconnected((u32, BlockHash)),
    Error(),
}
