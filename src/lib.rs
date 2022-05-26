mod mempool_space;

pub use mempool_space::api::MempoolSpaceWebSocketMessage;
use anyhow::{anyhow, Ok};
use bitcoin::Network;
use futures_util::{SinkExt, StreamExt};
use mempool_space::fetch_new_blocks;
use std::time::Duration;
use tokio_tungstenite::{connect_async_tls_with_config, tungstenite::protocol::Message};
use url::Url;

pub async fn fetch_data(network: Network, _data: Vec<String>) {
    // TODO: (@leonardo.lima) The data needs to be parsed in order to know which fn to use from mempool.space module
    fetch_new_blocks(network).await;
}

pub async fn fetch_blocks(url: Url, message: String) -> anyhow::Result<()> {
    let (mut websocket_stream, _ws_res) = connect_async_tls_with_config(url, None, None)
        .await
        .expect("failed to connect with url");
    println!("websocket handshake successfully completed!");

    if let Err(_) = websocket_stream.send(Message::text(message)).await {
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
                            let obj: MempoolSpaceWebSocketMessage = serde_json::from_str(&text).unwrap();
                            println!("{:?}", &obj);
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
