use clap::Parser;
use futures_util::{future, pin_mut, StreamExt};
use serde_json::json;
use std::env;
use tokio_tungstenite::{connect_async_tls_with_config, tungstenite::protocol::Message};

#[derive(Parser)]
#[clap(name = "CLI block explorer with mempool.space websocket - WIP")]
#[clap(author = "Leonardo L.")]
#[clap(version = "0.1")]
#[clap(about = "A work in progress CLI block explorer to be used with BDK, consuming data from mempool.space websocket.\n
                This an initial competency test for Summer of Bitcoin 2022", long_about = None)]
struct Cli {
    #[clap(default_value_t = String::from(env::var("DEFAULT_NETWORK").unwrap()), short, long)]
    network: String,
    #[clap(short, long)]
    blocks: bool,
    #[clap(short, long)]
    mempool_blocks: bool,
    #[clap(default_value_t = String::from(env::var("BLOCK_EXPLORER").unwrap()), short, long)]
    explorer: String,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let connect_address;
    if cli.network == "testnet" {
        connect_address = format!("wss://{}/testnet/api/v1/ws", cli.explorer);
    } else {
        connect_address = format!("wss://{}/api/v1/ws", cli.explorer);
    }

    let connect_url = url::Url::parse(&connect_address).unwrap();
    let (websocket_stream, _ws_res) = connect_async_tls_with_config(connect_url, None, None)
        .await.expect("failed to connect with url");
    println!("websocket handshake successfully completed!");

    let (write, read) = websocket_stream.split();

    let req_message = json!({
        "action": "want",
        "data": ["blocks", "stats", "mempool-blocks", "live-2h-chart"]
    });
    let req_message_str = serde_json::ser::to_string(&req_message).unwrap();

    let (stdin_tx, stdin_rx) = futures_channel::mpsc::unbounded();
    let stdin_message = stdin_tx.unbounded_send(Message::text(req_message_str)).unwrap(); 
    tokio::spawn(async move {
        stdin_message
    });

    let message_to_websocket = stdin_rx.map(Ok).forward(write);
    let ws_to_stdout = {
        read.for_each(|message| async {
            let str_data = message.unwrap().into_text().unwrap();
            let obj: serde_json::Value = serde_json::from_str(&str_data).unwrap();
            println!("{}", serde_json::to_string_pretty(&obj).unwrap());
        })
    };

    pin_mut!(message_to_websocket, ws_to_stdout);
    future::select(message_to_websocket, ws_to_stdout).await;

}
