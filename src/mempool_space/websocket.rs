use super::api::{
    BlockEvent, MempoolSpaceWebSocketMessage, MempoolSpaceWebSocketRequestData,
    MempoolSpaceWebSocketRequestMessage,
};

use anyhow::{anyhow, Ok};
use async_stream::stream;
use futures_util::stream::Stream;
use futures_util::{SinkExt, StreamExt};
use std::time::Duration;
use tokio_tungstenite::connect_async_tls_with_config;
use tokio_tungstenite::tungstenite::protocol::Message;
use url::Url;

pub async fn subscribe_to_blocks(url: &Url) -> anyhow::Result<impl Stream<Item = BlockEvent>> {
    log::info!("starting websocket handshake [url {}]", url);
    let (mut websocket_stream, websocket_response) =
        connect_async_tls_with_config(url, None, None).await?;

    log::info!("websocket handshake successfully completed!");
    log::info!(
        "handshake completed with response: {:?}",
        websocket_response
    );

    let message = build_websocket_request_message(&MempoolSpaceWebSocketRequestData::Blocks);
    let item = serde_json::to_string(&message).unwrap();
    if (websocket_stream.send(Message::text(&item)).await).is_err() {
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
                                log::debug!("[Message::Text {}]", text);
                                let res_message: MempoolSpaceWebSocketMessage = serde_json::from_str(&text).unwrap();
                                yield BlockEvent::Connected(res_message.block);
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

fn build_websocket_request_message(
    data: &MempoolSpaceWebSocketRequestData,
) -> MempoolSpaceWebSocketRequestMessage {
    let mut message = MempoolSpaceWebSocketRequestMessage {
        action: String::from("want"),
        data: vec![],
    };

    match data {
        MempoolSpaceWebSocketRequestData::Blocks => message.data.push(String::from("blocks")),
        MempoolSpaceWebSocketRequestData::MempoolBlocks => {
            message.data.push(String::from("mempool-blocks"))
        }
        // FIXME: (@leonardo.lima) fix this track-address to use different struct
        MempoolSpaceWebSocketRequestData::TrackAddress(..) => { /* ignore */ }
    }
    message
}
