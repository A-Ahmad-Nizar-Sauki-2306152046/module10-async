// Copyright 2023 Google LLC
// SPDX-License-Identifier: Apache-2.0
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// ANCHOR: solution
// ANCHOR: setup
use futures_util::sink::SinkExt;
use futures_util::stream::StreamExt;
use std::collections::HashMap;
use std::error::Error;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::broadcast::{channel, Sender};
use tokio_websockets::{Message, ServerBuilder, WebSocketStream};
use serde::{Deserialize, Serialize};
// ANCHOR_END: setup

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct IncomingMessage {
    message_type: String,
    data: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct OutgoingUsers {
    message_type: String,
    data_array: Vec<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct OutgoingMessage {
    message_type: String,
    data: String,
}

#[derive(Serialize)]
struct MessageData {
    from: String,
    message: String,
}

type Users = Arc<Mutex<HashMap<SocketAddr, String>>>;

// ANCHOR: handle_connection
async fn handle_connection(
    addr: SocketAddr,
    mut ws_stream: WebSocketStream<TcpStream>,
    bcast_tx: Sender<String>,
    users: Users,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    // ANCHOR_END: handle_connection

    let mut bcast_rx = bcast_tx.subscribe();

    // A continuous loop for concurrently performing two tasks: (1) receiving
    // messages from `ws_stream` and broadcasting them, and (2) receiving
    // messages on `bcast_rx` and sending them to the client.
    loop {
        tokio::select! {
            incoming = ws_stream.next() => {
                match incoming {
                    Some(Ok(msg)) => {
                        if let Some(text) = msg.as_text() {
                            println!("From client {addr:?}: {text}");
                            if let Ok(parsed) = serde_json::from_str::<IncomingMessage>(text) {
                                match parsed.message_type.as_str() {
                                    "register" => {
                                        if let Some(username) = parsed.data {
                                            users.lock().unwrap().insert(addr, username);
                                            let user_list: Vec<String> = users
                                                .lock().unwrap().values().cloned().collect();
                                            let response = serde_json::to_string(&OutgoingUsers {
                                                message_type: "users".to_string(),
                                                data_array: user_list,
                                            })?;
                                            bcast_tx.send(response)?;
                                        }
                                    }
                                    "message" => {
                                        if let Some(content) = parsed.data {
                                            let from = users.lock().unwrap()
                                                .get(&addr).cloned()
                                                .unwrap_or_else(|| "unknown".to_string());
                                            let msg_data = serde_json::to_string(&MessageData {
                                                from,
                                                message: content,
                                            })?;
                                            let response = serde_json::to_string(&OutgoingMessage {
                                                message_type: "message".to_string(),
                                                data: msg_data,
                                            })?;
                                            bcast_tx.send(response)?;
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                    Some(Err(err)) => return Err(err.into()),
                    None => {
                        // Client disconnected, remove from users
                        users.lock().unwrap().remove(&addr);
                        let user_list: Vec<String> = users
                            .lock().unwrap().values().cloned().collect();
                        let response = serde_json::to_string(&OutgoingUsers {
                            message_type: "users".to_string(),
                            data_array: user_list,
                        })?;
                        let _ = bcast_tx.send(response);
                        return Ok(());
                    }
                }
            }
            msg = bcast_rx.recv() => {
                ws_stream.send(Message::text(msg?)).await?;
            }
        }
    }
    // ANCHOR: main
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let (bcast_tx, _) = channel(16);
    let users: Users = Arc::new(Mutex::new(HashMap::new()));

    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("listening on port 8080");

    loop {
        let (socket, addr) = listener.accept().await?;
        println!("New connection from {addr:?}");
        let bcast_tx = bcast_tx.clone();
        let users = users.clone();
        tokio::spawn(async move {
            // Wrap the raw TCP stream into a websocket.
            let (_req, ws_stream) = ServerBuilder::new().accept(socket).await?;

            handle_connection(addr, ws_stream, bcast_tx, users).await
        });
    }
}
// ANCHOR_END: main
