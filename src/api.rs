use bitcoin::{Address, BlockHash, BlockHeader, TxMerkleNode};

#[derive(serde::Deserialize, Clone, Debug, Copy)]
pub struct BlockExtended {
    pub id: BlockHash,
    pub height: u32,
    pub version: i32,
    #[serde(alias = "previousblockhash")]
    pub prev_blockhash: BlockHash,
    pub merkle_root: TxMerkleNode,
    #[serde(alias = "timestamp")]
    pub time: u32,
    pub bits: u32,
    pub nonce: u32,
    // add new fields if needed
}

// FIXME: (@leonardo.lima) Should this use serde_json or some other approach instead ?
impl From<BlockExtended> for BlockHeader {
    fn from(extended: BlockExtended) -> BlockHeader {
        return BlockHeader {
            version: (extended.version),
            prev_blockhash: (extended.prev_blockhash),
            merkle_root: (extended.merkle_root),
            time: (extended.time),
            bits: (extended.bits),
            nonce: (extended.nonce),
        };
    }
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
    Connected(BlockHeader),
    Disconnected((u32, BlockHash)),
    Error(),
}
