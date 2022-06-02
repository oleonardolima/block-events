use bitcoin::{Address, Network};
use block_events::{fetch_data_stream, BlockEvent, MempoolSpaceWebSocketRequestData};
use clap::{ArgGroup, Parser, Subcommand};
use futures_util::{pin_mut, StreamExt};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Parser)]
#[clap(name = "block-events-cli")]
#[clap(author = "Leonardo Souza <leonardolimasza@gmail.com>, LLFourn <lloyd.fourn@gmail.com>")]
#[clap(version = "0.1.0")]
#[clap(
    long_about = "A CLI interface and tool to use with the block-events library.\n
This a work in progress project for Summer of Bitcoin 2022."
)]

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
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let data = build_request_data(&cli);
    let network = cli.network;

    let data_stream = fetch_data_stream(&network, &data).await?;

    pin_mut!(data_stream);

    while let Some(data) = data_stream.next().await {
        match data {
            BlockEvent::Connected(block) => {
                println!("received following event: Block Connected: {:#?}", block);
            }
            BlockEvent::Disconnected((height, block_hash)) => {
                println!("received following event: Block Disconnected: [height {:#?}] [block_hash: {:#?}]", height, block_hash);
            }
            BlockEvent::Error() => {
                eprint!("ERR: received an error from the data_stream");
            }
        }
    }

    Ok(())
}

#[allow(deprecated)]
fn build_request_data(cli: &Cli) -> MempoolSpaceWebSocketRequestData {
    match &cli.command {
        Commands::AddressTracking { address } => {
            return MempoolSpaceWebSocketRequestData::TrackAddress(
                Address::from_str(address.as_str()).unwrap(),
            );
        }
        Commands::DataStream { blocks, .. } => {
            if *blocks {
                return MempoolSpaceWebSocketRequestData::Blocks;
            }
            MempoolSpaceWebSocketRequestData::MempoolBlocks
        }
    }
}
