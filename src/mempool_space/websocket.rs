use anyhow::{anyhow, Ok};
use futures_util::{SinkExt, StreamExt};
use std::{time::Duration};
use super::api::{MempoolSpaceWebSocketMessage, MempoolSpaceWebSocketRequestMessage, BlockExtended};
use tokio_tungstenite::connect_async_tls_with_config;
use tokio_tungstenite::tungstenite::protocol::Message;
use url::Url;

use async_stream::stream;
use futures_util::stream::Stream;

pub async fn publish_message(url: Url, message: &MempoolSpaceWebSocketRequestMessage) -> anyhow::Result<impl Stream<Item = BlockExtended>> {
    let (mut websocket_stream, websocket_response) =
        connect_async_tls_with_config(&url, None, None)
        .await
        .expect(&format!("failed to connect with url: {}", &url));

    log::info!("websocket handshake successfully completed!");
    log::info!("handshake completed with response: {:?}", websocket_response);

    let item = serde_json::to_string(&message).unwrap();
    if let Err(_) = websocket_stream.send(Message::text(&item)).await {
        log::error!("failed to publish first message to websocket");
        return Err(anyhow!("failed to publish first message to websocket"));
    };
    log::info!("published message: {:#?}, successfully!", &item);

    // need to ping every so often to keep the websocket connection alive
    let mut pinger = tokio::time::interval(Duration::from_secs(60));

    let stream = stream! {
        loop {
            tokio::select! {
                message = websocket_stream.next() => {
                    if let Some(message) = message {
                        match message.unwrap() {
                            Message::Text(text) => {
                                let obj: MempoolSpaceWebSocketMessage = serde_json::from_str(&text).unwrap();
                                let block = obj.block;
                                yield block;
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
                    log::info!("pinging to websocket to keep connection alive");
                    websocket_stream.send(Message::Ping(vec![])).await.unwrap()
                }
            }
        }
    };
    Ok(stream)
}
