use anyhow::anyhow;
use clap::{ArgGroup, Parser};
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::{env, time::Duration};
use tokio_tungstenite::{connect_async_tls_with_config, tungstenite::protocol::Message};
use either::*;

#[derive(Parser)]
#[clap(name = "CLI block explorer with mempool.space websocket - WIP")]
#[clap(author = "Leonardo L.")]
#[clap(version = "0.1")]
#[clap(about = "A work in progress CLI block explorer to be used with BDK, consuming data from mempool.space websocket.\n
                This an initial competency test for Summer of Bitcoin 2022", long_about = None)]
#[clap(group(ArgGroup::new("flags")
                .required(true)
                .args(&["blocks-data", "track-address"]),
            ))]

struct Cli {
    #[clap(short, long, group = "blocks")]
    blocks_data: bool,

    #[clap(short, long, value_name = "ADDRESS")]
    track_address: String,

    #[clap(long, requires = "blocks")]
    no_blocks: bool,
 
    #[clap(long, requires = "blocks")]
    no_mempool_blocks: bool,
 
    #[clap(short, long)]
    endpoint: Option<String>,
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

    let req_message;
    let message = build_request_message(&cli);
    if message.is_right() {
        req_message = serde_json::to_string(&build_request_message(&cli).unwrap_right()).unwrap();
    } else {
        req_message = serde_json::to_string(&build_request_message(&cli).unwrap_left()).unwrap();
    }

    println!("[req-message] {:?}", req_message);

    let connect_address = format!(
        "wss://{}/v1/ws",
        cli.endpoint
            .or(env::var("MEMPOOL_ENDPOINT").ok())
            .unwrap_or("mempool.space/api".to_string())
    );

    let connect_url = url::Url::parse(&connect_address).unwrap();
    let (mut websocket_stream, _ws_res) = connect_async_tls_with_config(connect_url, None, None)
        .await
        .expect("failed to connect with url");
    println!("websocket handshake successfully completed!");

    if let Err(_) = websocket_stream.send(Message::text(req_message)).await {
        return Err(anyhow!("Failed to send first message to websocket"));
    }

    // need to ping every so often to keep websocket alive
    let mut pinger = tokio::time::interval(Duration::from_secs(60));

    loop {
        tokio::select! {
            message = websocket_stream.next() => {
                if let Some(message) = message {
                    match message? {
                        Message::Text(text) => {
                            let obj: serde_json::Value = serde_json::from_str(&text).unwrap();
                            println!("{}", serde_json::to_string_pretty(&obj).unwrap());
                        },
                        Message::Close(_) => {
                            eprintln!("websocket closing gracefully");
                            break;
                        },
                        Message::Binary(_) => {
                            eprintln!("unexpected binary message");
                            break;
                        },
                        _ => { /*ignore*/ }
                    }
                }
            }
            _ = pinger.tick() => {
                websocket_stream.send(Message::Ping(vec![])).await.unwrap()
            }
        }
    }

    Ok(())
}

fn build_request_message(cli: &Cli) -> Either<BlockDataMessage, TrackAddressMessage>{

    if cli.blocks_data {
        let mut data = vec![];

        if !cli.no_mempool_blocks {
            data.push(String::from("mempool-blocks"));
        }

        if !cli.no_blocks {
            data.push(String::from("blocks"));
        }

        return either::Left(BlockDataMessage {action: String::from("want"), data: data});
    } else {
        return either::Right(TrackAddressMessage {track_address: String::from(&cli.track_address)});
    }
}