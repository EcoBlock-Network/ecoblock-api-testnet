use axum::{
    extract::{ws::WebSocket, WebSocketUpgrade},
    response::Response,
};
use futures::{sink::SinkExt, stream::StreamExt};
use serde_json::json;
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
    time::{Duration, Instant},
};
use tokio::time::interval;
use uuid::Uuid;

#[derive(Clone)]
pub struct WebSocketManager {
    connections: Arc<RwLock<HashMap<Uuid, WebSocketConnection>>>,
}

struct WebSocketConnection {
    id: Uuid,
    connected_at: Instant,
}

impl WebSocketManager {
    pub fn new() -> Self {
        Self {
            connections: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn connection_count(&self) -> usize {
        self.connections.read().unwrap().len()
    }

    pub async fn handle_upgrade(
        &self,
        ws: WebSocketUpgrade,
        network_stats: Arc<RwLock<crate::models::ApiNetworkStats>>,
    ) -> Response {
        let manager = self.clone();
        ws.on_upgrade(
            move |socket| async move { manager.handle_socket(socket, network_stats).await },
        )
    }

    async fn handle_socket(
        &self,
        socket: WebSocket,
        network_stats: Arc<RwLock<crate::models::ApiNetworkStats>>,
    ) {
        let connection_id = Uuid::new_v4();
        let connection = WebSocketConnection {
            id: connection_id,
            connected_at: Instant::now(),
        };

        {
            let mut connections = self.connections.write().unwrap();
            connections.insert(connection_id, connection);
        }

        let (mut sender, mut receiver) = socket.split();
        let mut interval = interval(Duration::from_secs(2));

        loop {
            tokio::select! {
                msg = receiver.next() => {
                    if let Some(msg) = msg {
                        match msg {
                            Ok(msg) => {
                                if matches!(msg, axum::extract::ws::Message::Close(_)) {
                                    break;
                                }
                                if let Ok(_text) = msg.to_text() {
                                }
                            }
                            Err(_e) => {
                                break;
                            }
                        }
                    } else {
                        break;
                    }
                }

                _ = interval.tick() => {
                    let stats = {
                        let guard = network_stats.read().unwrap();
                        guard.clone()
                    };

                    let update = json!({
                        "type": "network_update",
                        "timestamp": chrono::Utc::now().to_rfc3339(),
                        "data": stats
                    });

                    if let Err(_e) = sender.send(axum::extract::ws::Message::Text(update.to_string())).await {
                        break;
                    }
                }
            }
        }

        {
            let mut connections = self.connections.write().unwrap();
            connections.remove(&connection_id);
        }
    }

    pub async fn broadcast_block_created(&self, block: &crate::models::BlockInfo) {
        let _message = json!({
            "type": "block_created",
            "timestamp": chrono::Utc::now().to_rfc3339(),
            "data": block
        });
    }

    pub async fn broadcast_network_event(&self, event: &str, data: serde_json::Value) {
        let _message = json!({
            "type": "network_event",
            "event": event,
            "timestamp": chrono::Utc::now().to_rfc3339(),
            "data": data
        });
    }
}

impl Default for WebSocketManager {
    fn default() -> Self {
        Self::new()
    }
}
