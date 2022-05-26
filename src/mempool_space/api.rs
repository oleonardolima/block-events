// TODO: (@leonardo.lima) Add and fix all the missing types
// #[derive(serde::Deserialize, Debug)]
// pub struct Pool {
//     id: String,
//     name: String,
//     slug: String,
// }

// TODO: (@leonardo.lima) Add and fix all the missing types
// #[derive(serde::Deserialize, Debug)]
// pub struct BlockExtension {
//   pub totalFees: String,
//   pub medianFee: String,
//   pub feeRange: String,
//   pub reward: String,
//   pub coinbaseTx: String,
//   pub matchRate: String,
//   pub pool: Pool,
//   pub avgFee: String,
//   pub avgFeeRate: String,
//   pub coinbaseRaw: String,
// }

// TODO: (@leonardo.lima) Add and fix all the missing types
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

// TODO: (@leonardo.lima) Add and fix all the missing types
// #[derive(serde::Deserialize, Debug)]
// pub struct MempoolInfo {
//   pub loaded: String,              //  (boolean) True if the mempool is fully loaded
//   pub size: String,                //  (numeric) Current tx count
//   pub bytes: String,               //  (numeric) Sum of all virtual transaction sizes as defined in BIP 141.
//   pub usage: String,               //  (numeric) Total memory usage for the mempool
//   pub total_fee: String,           //  (numeric) Total fees of transactions in the mempool
//   pub maxmempool: String,          //  (numeric) Maximum memory usage for the mempool
//   pub mempoolminfee: String,       //  (numeric) Minimum fee rate in BTC/kB for tx to be accepted.
//   pub minrelaytxfee: String,       //  (numeric) Current minimum relay fee for transactions
// }

// TODO: (@leonardo.lima) Add and fix all the missing types
// #[derive(serde::Deserialize, Debug)]
// pub struct DifficultyAdjustment {
//   pub progressPercent: String,
//   pub difficultyChange: String,
//   pub estimatedRetargetDate: String,
//   pub remainingBlocks: String,
//   pub remainingTime: String,
//   pub previousRetarget: String,
//   pub nextRetargetHeight: String,
//   pub timeAvg: String,
//   pub timeOffset: String,
// }

#[derive(serde::Deserialize, Debug)]
pub struct MempoolSpaceWebSocketMessage {
  pub block: BlockExtended,
  // pub mempoolInfo: MempoolInfo,
  // pub da: DifficultyAdjustment,
}

#[derive(serde::Serialize, Debug)]
pub struct MempoolSpaceWebSocketRequestMessage {
  pub action: String,
  pub data: Vec<String>,
}