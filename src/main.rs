use ecoblock_api::server::start_api_server;
use ecoblock_network::NetworkNode;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logger
    env_logger::init();
    
    println!("🚀 Starting EcoBlock API Server...");
    
    // Create network node
    let network_node = Arc::new(NetworkNode::new(3000, 3002).await?);
    
    // Start the server
    let addr = "127.0.0.1:3000".parse()?;
    start_api_server(network_node, addr).await?;
    Ok(())
}
