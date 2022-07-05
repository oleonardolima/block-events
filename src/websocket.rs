// Block Events Library
// Written in 2022 by Leonardo Lima <> and Lloyd Fournier <>
//
// This file is licensed under the Apache License, Version 2.0 <LICENSE-APACHE
// or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// You may not use this file except in accordance with one or both of these
// licenses.

//! WebSocket module for mempool.space
//! It has functions to connect and create a new WebSocket client, and also subscribe for only new block events

use crate::api::BlockExtended;

use super::api::{
    MempoolSpaceWebSocketMessage, MempoolSpaceWebSocketRequestData,
    MempoolSpaceWebSocketRequestMessage,
};

use anyhow::{anyhow, Ok};
use async_stream::stream;
use futures_util::stream::Stream;
use futures_util::{SinkExt, StreamExt};
use std::time::Duration;
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::protocol::Message;
use tokio_tungstenite::{connect_async_tls_with_config, MaybeTlsStream, WebSocketStream};
use url::Url;

/// Create a new WebSocket client for given base url and initial message
///
/// It uses `tokio_tungestenite` crate and produces `WebSocketStream` to be handled and treated by caller
async fn websocket_client(
    base_url: &Url,
    message: String,
) -> anyhow::Result<WebSocketStream<MaybeTlsStream<TcpStream>>> {
    let url = url::Url::parse(format!("ws://{}/ws", base_url).as_str()).unwrap();
    log::info!("starting websocket handshake with url={}", url);

    // TODO: (@leonardo.lima) It's needed to infer the tls security from network, or feature ?
    let (mut websocket_stream, websocket_response) =
        connect_async_tls_with_config(url, None, None).await?;

    log::info!("websocket handshake successfully completed!");
    log::info!("handshake completed with response={:?}", websocket_response);

    if (websocket_stream.send(Message::text(&message)).await).is_err() {
        log::error!("failed to publish first message to websocket");
        return Err(anyhow!("failed to publish first message to websocket"));
    };
    log::info!("published message: {:#?}, successfully!", &message);
    Ok(websocket_stream)
}

/// Connects to mempool.space WebSocket client and listen to new messages producing a stream of [`BlockExtended`] candidates
pub async fn subscribe_to_blocks(
    base_url: &Url,
) -> anyhow::Result<impl Stream<Item = BlockExtended>> {
    let init_message = serde_json::to_string(&build_websocket_request_message(
        &MempoolSpaceWebSocketRequestData::Blocks,
    ))
    .unwrap();

    let mut ws_stream = websocket_client(base_url, init_message).await.unwrap();

    // need to ping every so often to keep the websocket connection alive
    let mut pinger = tokio::time::interval(Duration::from_secs(60));

    let stream = stream! {
        loop {
            tokio::select! {
                message = ws_stream.next() => {
                    if let Some(message) = message {
                        match message.unwrap() {
                            Message::Text(text) => {
                                let parse_ws_msg = || -> anyhow::Result<()> {
                                    let _: MempoolSpaceWebSocketMessage = serde_json::from_str(&text)?;
                                    Ok(())
                                };
                                if let Err(_) = parse_ws_msg() {
                                    continue
                                }
                                let res_msg: MempoolSpaceWebSocketMessage = serde_json::from_str(&text).unwrap();
                                log::debug!("{:?}", res_msg.block);
                                yield res_msg.block;
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
                    ws_stream.send(Message::Ping(vec![])).await.unwrap() // TODO: (@leonardo.lima) Should this use a mempool expected ping message instead ?
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
