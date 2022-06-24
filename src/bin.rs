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

    let websocket_url = url::Url::parse(&match cli.network {
        Network::Bitcoin => "wss://mempool.space/api/v1/ws".to_string(),
        Network::Regtest => "ws://localhost/api/v1/ws".to_string(),
        network => format!("wss://mempool.space/{}/api/v1/ws", network),
    })
    .unwrap();

    let data_stream = block_events::websocket::subscribe_to_blocks(&websocket_url).await?;

    pin_mut!(data_stream);

    while let Some(data) = data_stream.next().await {
        match data {
            BlockEvent::Connected(block) => {
                println!("[Event][Block Connected]\n {:#?}", block);
            }
            BlockEvent::Disconnected((height, block_hash)) => {
                println!(
                    "[Event][Block Disconnected] [height {:#?}] [block_hash: {:#?}]",
                    height, block_hash
                );
            }
            BlockEvent::Error() => {
                eprint!("ERROR: received an error from the data_stream");
            }
        }
    }

    Ok(())
}
