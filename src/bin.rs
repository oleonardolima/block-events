use bitcoin::Network;
use block_explorer_cli::{fetch_data_stream, MempoolSpaceWebSocketRequestData};
use clap::{ArgGroup, Parser, Subcommand};
use serde::{Deserialize, Serialize};

#[derive(Parser)]
#[clap(name = "CLI block explorer with mempool.space websocket - WIP")]
#[clap(author = "Leonardo L.")]
#[clap(version = "0.1")]
#[clap(about = "A work in progress CLI block explorer to be used with BDK, consuming data from mempool.space websocket.\n
                This an initial competency test for Summer of Bitcoin 2022", long_about = None)]

struct Cli {
    #[clap(subcommand)]
    command: Commands,

    #[clap(short, long, default_value = "testnet")]
    network: Network,
}

#[derive(Debug, Subcommand)]
enum Commands {
    // track address feature from mempool.space ws
    AddressTracking {
        #[clap(short, long)]
        address: String,
    },

    // subscribe and fetch new blocks related data
    #[clap(group(ArgGroup::new("data-stream")
                    .required(true)
                    .args(&["blocks", "mempool-blocks"])))]
    DataStream {
        // new blocks data only
        #[clap(long)]
        blocks: bool,

        // new mempool-blocks data only
        #[deprecated]
        #[clap(long)]
        mempool_blocks: bool,
    },
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
struct BlockDataMessage {
    action: String,
    data: Vec<String>,
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
struct TrackAddressMessage {
    track_address: String,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let data = build_request_data(&cli);
    let network = cli.network;

    fetch_data_stream(&network, &data).await.unwrap();
}

#[allow(deprecated)]
fn build_request_data(cli: &Cli) -> MempoolSpaceWebSocketRequestData {
    match &cli.command {
        Commands::AddressTracking { address } => {
            return MempoolSpaceWebSocketRequestData::TrackAddress(String::from(address));
        }
        Commands::DataStream { blocks, .. } => {
            if *blocks {
                return MempoolSpaceWebSocketRequestData::Blocks
            }
            return MempoolSpaceWebSocketRequestData::MempoolBlocks;
        }
    }
}
