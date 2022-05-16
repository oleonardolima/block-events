use clap::{Subcommand, Parser};
use serde::{Deserialize, Serialize};
use std::{env};
use block_explorer_cli::fetch_blocks;

#[derive(Parser)]
#[clap(name = "CLI block explorer with mempool.space websocket - WIP")]
#[clap(author = "Leonardo L.")]
#[clap(version = "0.1")]
#[clap(about = "A work in progress CLI block explorer to be used with BDK, consuming data from mempool.space websocket.\n
                This an initial competency test for Summer of Bitcoin 2022", long_about = None)]

struct Cli {
    #[clap(subcommand)]
    command: Commands,

    #[clap(short, long)]
    endpoint: Option<String>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    // track address
    TrackAddress {
        // address to track
        #[clap(short, long)]
        address: String,
    },
    // fetch all new blocks
    BlocksData {
        // remove blocks subscription
        #[clap(long)]
        no_blocks: bool,

        // remove mempool blocks subscription
        #[clap(long)]
        no_mempool_blocks: bool,
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

    let req_message = build_request_message(&cli);

    // TODO: (@leonardo.lima) extract this to a fn based on connection type (WS, HTTP, ...)
    let connect_address = format!(
        "wss://{}/v1/ws",
        cli.endpoint
            .or(env::var("MEMPOOL_ENDPOINT").ok())
            .unwrap_or("mempool.space/api".to_string())
    );

    let connect_url = url::Url::parse(&connect_address).unwrap();

    fetch_blocks(connect_url, req_message).await.unwrap();

}

fn build_request_message(cli: &Cli) -> String {

    match &cli.command {
        Commands::TrackAddress { address } => {
            return serde_json::to_string(&(TrackAddressMessage {track_address: String::from(address)})).unwrap();
        }
        Commands::BlocksData { no_blocks, no_mempool_blocks} => {
            let mut data = vec![];

            if !no_mempool_blocks {
                data.push(String::from("mempool-blocks"));
            }

            if !no_blocks {
                data.push(String::from("blocks"));
            }

            return serde_json::to_string(&(BlockDataMessage {action: String::from("want"), data: data})).unwrap();
        }
    }
}