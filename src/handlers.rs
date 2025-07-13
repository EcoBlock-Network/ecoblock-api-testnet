use axum::{
    extract::{Path, State, WebSocketUpgrade},
    http::StatusCode,
    response::{Json, Response},
    routing::{get, post},
    Router,
};
use serde_json::json;
use std::sync::{Arc, RwLock};

use crate::models::*;
use crate::websocket::WebSocketManager;
use ecoblock_core::TangleBlockData;
use ecoblock_crypto::keys::keypair::CryptoKeypair;
use ecoblock_network::NetworkNode;
use ecoblock_storage::tangle::block::TangleBlock;

pub type SharedState = Arc<NetworkNode>;

pub struct AppState {
    pub network_node: Arc<NetworkNode>,
    pub websocket_manager: WebSocketManager,
    pub network_stats: Arc<RwLock<crate::models::ApiNetworkStats>>,
}

pub fn create_router(state: SharedState) -> Router {
    let websocket_manager = WebSocketManager::new();
    let network_stats = Arc::new(RwLock::new(crate::models::ApiNetworkStats {
        info: crate::models::NetworkInfo {
            node_id: "test-node".to_string(),
            peer_count: 0,
            block_count: 0,
            uptime: 0,
            status: "active".to_string(),
            version: "0.1.0".to_string(),
            network_type: "test".to_string(),
        },
        peers: vec![],
        metrics: crate::models::NetworkMetrics {
            total_blocks: 0,
            blocks_per_minute: 0.0,
            average_latency: 0.0,
            network_health: 1.0,
            active_peers: 0,
            total_peers: 0,
            active_connections: 0,
            messages_sent: 0,
            messages_received: 0,
            bytes_sent: 0,
            bytes_received: 0,
        },
    }));

    let app_state = Arc::new(AppState {
        network_node: state.clone(),
        websocket_manager,
        network_stats,
    });

    Router::new()
        .route("/ws", get(websocket_handler))
        .route("/api/network/info", get(get_network_info))
        .route("/api/network/peers", get(get_peers))
        .route("/api/network/metrics", get(get_network_metrics))
        .route("/api/network/stats", get(get_network_stats))
        .route("/api/blocks", get(get_blocks))
        .route("/api/blocks/:hash", get(get_block))
        .route("/api/blocks", post(create_block))
        .route("/api/blocks/:hash/send", post(send_block))
        .route("/api/simulation/config", get(get_simulation_config))
        .route("/api/simulation/config", post(set_simulation_config))
        .route("/api/simulation/start", post(start_simulation))
        .route("/api/simulation/stop", post(stop_simulation))
        .route("/api/simulation/status", get(get_simulation_status))
        .route("/api/health", get(health_check))
        .route("/api/version", get(get_version))
        .with_state(app_state)
}

// WebSocket handler
pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
) -> Response {
    state
        .websocket_manager
        .handle_upgrade(ws, state.network_stats.clone())
        .await
}

pub async fn get_network_info(
    State(state): State<Arc<AppState>>,
) -> Result<Json<ApiResponse<NetworkInfo>>, StatusCode> {
    let stats = state.network_node.get_network_stats().await;

    let info = NetworkInfo {
        node_id: stats.node_id.0.to_string(),
        peer_count: stats.peer_count,
        block_count: stats.block_count,
        uptime: stats.uptime,
        status: "active".to_string(),
        version: "0.1.0".to_string(),
        network_type: "mesh".to_string(),
    };

    Ok(Json(ApiResponse::success(info)))
}

pub async fn get_peers(
    State(state): State<Arc<AppState>>,
) -> Result<Json<ApiResponse<Vec<PeerInfo>>>, StatusCode> {
    let peers = state.network_node.peer_discovery.get_peers().await;

    let peer_infos: Vec<PeerInfo> = peers
        .into_iter()
        .map(|(id, info)| PeerInfo {
            id: id.0.to_string(),
            address: info.address.to_string(),
            last_seen: info.last_seen,
            is_connected: info.is_connected,
            latency: info.latency,
        })
        .collect();

    Ok(Json(ApiResponse::success(peer_infos)))
}

pub async fn get_network_metrics(
    State(state): State<Arc<AppState>>,
) -> Result<Json<ApiResponse<NetworkMetrics>>, StatusCode> {
    let stats = state.network_node.get_network_stats().await;
    let peers = state.network_node.peer_discovery.get_peers().await;

    let total_latency: u64 = peers.values().filter_map(|p| p.latency).sum();

    let active_peers = peers.len();
    let average_latency = if active_peers > 0 {
        total_latency as f64 / active_peers as f64
    } else {
        0.0
    };

    let metrics = NetworkMetrics {
        total_blocks: stats.block_count,
        blocks_per_minute: 0.0,
        average_latency,
        network_health: if active_peers > 0 { 1.0 } else { 0.0 },
        active_peers,
        total_peers: peers.len(),
        active_connections: active_peers,
        messages_sent: 0,
        messages_received: 0,
        bytes_sent: 0,
        bytes_received: 0,
    };

    Ok(Json(ApiResponse::success(metrics)))
}

pub async fn get_network_stats(
    State(state): State<Arc<AppState>>,
) -> Result<Json<ApiResponse<serde_json::Value>>, StatusCode> {
    let stats = state.network_node.get_network_stats().await;
    let peers = state.network_node.peer_discovery.get_peers().await;

    let response = json!({
        "node_id": stats.node_id.0.to_string(),
        "peer_count": stats.peer_count,
        "block_count": stats.block_count,
        "uptime": stats.uptime,
        "peers": peers.into_iter().map(|(id, info)| {
            json!({
                "id": id.0.to_string(),
                "address": info.address.to_string(),
                "last_seen": info.last_seen,
                "is_connected": info.is_connected,
                "latency": info.latency
            })
        }).collect::<Vec<_>>()
    });

    Ok(Json(ApiResponse::success(response)))
}

pub async fn get_blocks(
    State(state): State<Arc<AppState>>,
) -> Result<Json<ApiResponse<Vec<BlockInfo>>>, StatusCode> {
    let blocks = state.network_node.block_cache.read().await;

    let block_infos: Vec<BlockInfo> = blocks
        .values()
        .map(|block| BlockInfo {
            hash: block.id.clone(),
            timestamp: block.data.data.timestamp,
            sensor_data: block.data.data.clone(),
            signature: block.signature.0.clone(),
            parent_hashes: block.data.parents.clone(),
        })
        .collect();

    Ok(Json(ApiResponse::success(block_infos)))
}

pub async fn get_block(
    State(state): State<Arc<AppState>>,
    Path(hash): Path<String>,
) -> Result<Json<ApiResponse<BlockInfo>>, StatusCode> {
    let blocks = state.network_node.block_cache.read().await;

    match blocks.get(&hash) {
        Some(block) => {
            let block_info = BlockInfo {
                hash: block.id.clone(),
                timestamp: block.data.data.timestamp,
                sensor_data: block.data.data.clone(),
                signature: block.signature.0.clone(),
                parent_hashes: block.data.parents.clone(),
            };
            Ok(Json(ApiResponse::success(block_info)))
        }
        None => Err(StatusCode::NOT_FOUND),
    }
}

pub async fn create_block(
    State(state): State<Arc<AppState>>,
    Json(request): Json<CreateBlockRequest>,
) -> Result<Json<ApiResponse<BlockInfo>>, StatusCode> {
    let keypair = CryptoKeypair::generate();

    // Get parent blocks from cache (select recent tips)
    let parents = {
        let cache = state.network_node.block_cache.read().await;
        let mut blocks: Vec<(&String, &TangleBlock)> = cache.iter().collect();

        // Sort by timestamp (most recent first)
        blocks.sort_by(|a, b| b.1.data.data.timestamp.cmp(&a.1.data.data.timestamp));

        // Select 1-2 most recent blocks as parents (tip selection algorithm)
        if blocks.is_empty() {
            vec![] // Genesis block has no parents
        } else if blocks.len() == 1 {
            vec![blocks[0].0.clone()] // Reference the single existing block
        } else {
            // Select the two most recent blocks as parents
            vec![blocks[0].0.clone(), blocks[1].0.clone()]
        }
    };

    let data = TangleBlockData {
        parents,
        data: request.sensor_data,
    };

    let block = TangleBlock::new(data, &keypair);

    {
        let mut cache = state.network_node.block_cache.write().await;
        cache.insert(block.id.clone(), block.clone());
    }

    let block_info = BlockInfo {
        hash: block.id.clone(),
        timestamp: block.data.data.timestamp,
        sensor_data: block.data.data.clone(),
        signature: block.signature.0.clone(),
        parent_hashes: block.data.parents.clone(),
    };

    Ok(Json(ApiResponse::success(block_info)))
}

pub async fn send_block(
    State(state): State<Arc<AppState>>,
    Path(hash): Path<String>,
    Json(request): Json<SendBlockRequest>,
) -> Result<Json<ApiResponse<String>>, StatusCode> {
    let blocks = state.network_node.block_cache.read().await;

    match blocks.get(&hash) {
        Some(block) => {
            if let Err(e) = state.network_node.broadcast_block(block.clone()).await {
                return Ok(Json(ApiResponse::error(format!(
                    "Failed to send block: {}",
                    e
                ))));
            }
            Ok(Json(ApiResponse::success(
                "Block sent successfully".to_string(),
            )))
        }
        None => Err(StatusCode::NOT_FOUND),
    }
}

pub async fn get_simulation_config() -> Result<Json<ApiResponse<SimulationConfig>>, StatusCode> {
    let config = SimulationConfig {
        topology: "mesh".to_string(),
        node_count: 10,
        block_count: 100,
        interval_ms: 1000,
    };
    Ok(Json(ApiResponse::success(config)))
}

pub async fn set_simulation_config(
    Json(config): Json<SimulationConfig>,
) -> Result<Json<ApiResponse<String>>, StatusCode> {
    Ok(Json(ApiResponse::success(
        "Configuration updated".to_string(),
    )))
}

pub async fn start_simulation() -> Result<Json<ApiResponse<String>>, StatusCode> {
    Ok(Json(ApiResponse::success("Simulation started".to_string())))
}

pub async fn stop_simulation() -> Result<Json<ApiResponse<String>>, StatusCode> {
    Ok(Json(ApiResponse::success("Simulation stopped".to_string())))
}

pub async fn get_simulation_status() -> Result<Json<ApiResponse<SimulationStatus>>, StatusCode> {
    let status = SimulationStatus {
        is_running: false,
        blocks_sent: 0,
        blocks_received: 0,
        start_time: None,
        duration: None,
    };
    Ok(Json(ApiResponse::success(status)))
}

pub async fn health_check() -> Result<Json<ApiResponse<String>>, StatusCode> {
    Ok(Json(ApiResponse::success("OK".to_string())))
}

pub async fn get_version() -> Result<Json<ApiResponse<serde_json::Value>>, StatusCode> {
    let version_info = json!({
        "version": env!("CARGO_PKG_VERSION"),
        "name": env!("CARGO_PKG_NAME"),
        "build_time": chrono::Utc::now(),
        "rust_version": "1.70.0"
    });
    Ok(Json(ApiResponse::success(version_info)))
}
