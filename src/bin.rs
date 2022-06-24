use bitcoin::Network;
use block_events::api::BlockEvent;
use clap::{ArgGroup, Parser, Subcommand};
use futures_util::{pin_mut, StreamExt};
use serde::{Deserialize, Serialize};

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
    env_logger::init();

    let cli = Cli::parse();

    // build the url by the network argument
    let url = url::Url::parse(&match cli.network {
        Network::Bitcoin => "wss://mempool.space/api/v1/ws".to_string(),
        Network::Regtest => "ws://localhost:8999/api/v1/ws".to_string(),
        network => format!("wss://mempool.space/{}/api/v1/ws", network),
    })
    .unwrap();

    // async fetch the data stream through the lib
    let block_events = block_events::websocket::subscribe_to_blocks(&url).await?;

    // consume and execute the code (current matching and printing) in async manner for each new block-event
    pin_mut!(block_events);
    while let Some(block_event) = block_events.next().await {
        match block_event {
            BlockEvent::Connected(block) => {
                println!("Connected BlockEvent: {:#?}", block);
            }
            BlockEvent::Disconnected((height, block_hash)) => {
                println!(
                    "Disconnected BlockEvent: [height {:#?}] [block_hash: {:#?}]",
                    height, block_hash
                );
            }
            BlockEvent::Error() => {
                eprint!("ERROR BlockEvent: received an error from the block-events stream");
            }
        }
    }
    Ok(())
}
