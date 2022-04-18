use clap::Parser;
use futures_util::{SinkExt, StreamExt};
use anyhow::anyhow;
use serde_json::json;
use std::{env, time::Duration};
use tokio_tungstenite::{connect_async_tls_with_config, tungstenite::protocol::Message};

#[derive(Parser)]
#[clap(name = "CLI block explorer with mempool.space websocket - WIP")]
#[clap(author = "Leonardo L.")]
#[clap(version = "0.1")]
#[clap(about = "A work in progress CLI block explorer to be used with BDK, consuming data from mempool.space websocket.\n
                This an initial competency test for Summer of Bitcoin 2022", long_about = None)]
struct Cli {
    #[clap(long)]
    no_blocks: bool,
    #[clap(long)]
    no_mempool_blocks: bool,
    #[clap(short, long)]
    endpoint: Option<String>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

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

    let mut data = vec![];
    if !cli.no_mempool_blocks {
        data.push("mempool-blocks");
    }
    if !cli.no_blocks {
        data.push("blocks");
    }

    let req_message = json!({
        "action": "want",
        "data": data
    });
    let req_message_str = serde_json::ser::to_string(&req_message).unwrap();

    if let Err(_) = websocket_stream.send(Message::text(req_message_str)).await {
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
