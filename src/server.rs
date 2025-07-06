use axum::Router;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
use anyhow::Result;

use crate::handlers::{create_router, AppState};
use crate::middleware::create_middleware_stack;
use crate::websocket::WebSocketManager;
use ecoblock_network::NetworkNode;

pub struct ApiServer {
    app: Router,
    addr: SocketAddr,
}

impl ApiServer {
    pub async fn new(network_node: Arc<NetworkNode>, bind_addr: SocketAddr) -> Result<Self> {
        let app = create_router(network_node)
            .layer(create_middleware_stack());
        
        Ok(Self {
            app,
            addr: bind_addr,
        })
    }

    pub async fn start(self) -> Result<()> {
        let listener = TcpListener::bind(self.addr).await?;
        
        axum::serve(listener, self.app)
            .await
            .map_err(|e| {
                anyhow::anyhow!("Server failed: {}", e)
            })
    }

    pub fn addr(&self) -> SocketAddr {
        self.addr
    }
}

pub async fn start_api_server(
    network_node: Arc<NetworkNode>,
    bind_addr: SocketAddr,
) -> Result<()> {
    let server = ApiServer::new(network_node, bind_addr).await?;
    server.start().await
}
