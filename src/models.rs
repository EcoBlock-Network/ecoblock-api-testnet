use serde::{Deserialize, Serialize};
use ecoblock_network::PeerId;
use ecoblock_core::SensorData;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: Option<String>,
    pub timestamp: DateTime<Utc>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            message: None,
            timestamp: Utc::now(),
        }
    }

    pub fn error(message: String) -> Self {
        Self {
            success: false,
            data: None,
            message: Some(message),
            timestamp: Utc::now(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkInfo {
    pub node_id: String,
    pub peer_count: usize,
    pub block_count: usize,
    pub uptime: u64,
    pub status: String,
    pub version: String,
    pub network_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerInfo {
    pub id: String,
    pub address: String,
    pub last_seen: u64,
    pub is_connected: bool,
    pub latency: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockInfo {
    pub hash: String,
    pub timestamp: u64,
    pub sensor_data: SensorData,
    pub signature: String,
    pub parent_hashes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkMetrics {
    pub total_blocks: usize,
    pub blocks_per_minute: f64,
    pub average_latency: f64,
    pub network_health: f64,
    pub active_peers: usize,
    pub total_peers: usize,
    pub active_connections: usize,
    pub messages_sent: usize,
    pub messages_received: usize,
    pub bytes_sent: usize,
    pub bytes_received: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateBlockRequest {
    pub sensor_data: SensorData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendBlockRequest {
    pub block_hash: String,
    pub target_peers: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationConfig {
    pub topology: String,
    pub node_count: usize,
    pub block_count: usize,
    pub interval_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationStatus {
    pub is_running: bool,
    pub blocks_sent: usize,
    pub blocks_received: usize,
    pub start_time: Option<DateTime<Utc>>,
    pub duration: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub hash: String,
    pub previous_hash: String,
    pub timestamp: String,
    pub sensor_data: SensorData,
    pub signature: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Peer {
    pub id: String,
    pub address: std::net::SocketAddr,
    pub connected_at: chrono::DateTime<chrono::Utc>,
    pub last_seen: chrono::DateTime<chrono::Utc>,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiNetworkStats {
    pub info: NetworkInfo,
    pub peers: Vec<Peer>,
    pub metrics: NetworkMetrics,
}
