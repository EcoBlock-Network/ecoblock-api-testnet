# ecoblock-api-testnet

**High-Performance REST API for EcoBlock Network Testnet**

Rust-based REST API server providing real-time network monitoring, WebSocket connections, and blockchain data access for the EcoBlock decentralized sensor network.

## Features

- **REST API Endpoints** : Complete network statistics and health monitoring
- **WebSocket Support** : Real-time updates and live data streaming  
- **High Performance** : Built with Axum and Tokio for maximum throughput
- **Network Integration** : Direct connection to EcoBlock network nodes
- **Clean Architecture** : Modular design with clear separation of concerns

## Quick Start

### Prerequisites
- Rust 1.70+
- Cargo package manager

### Installation

```bash
# Clone the repository
git clone https://github.com/EcoBlock-Network/ecoblock-api-testnet.git
cd ecoblock-api-testnet

# Build the project
cargo build --release

# Run the API server
cargo run --release
```

## API Endpoints

### Network Information
```
GET /api/health              # API health status
GET /api/network/stats       # Complete network statistics
GET /api/network/info        # Basic network information
GET /api/network/peers       # Connected peers list
GET /api/network/metrics     # Performance metrics
```

### Blocks and Data
```
GET /api/blocks              # Recent blocks list
GET /api/blocks/{id}         # Specific block details
POST /api/sensor-data        # Submit new sensor data
```

### WebSocket
```
WS /api/ws                   # Real-time network updates
```

## Configuration

### Environment Variables
Create a `.env` file:
```env
API_PORT=9000
NETWORK_TCP_PORT=9001
NETWORK_UDP_PORT=9002
LOG_LEVEL=info
```

### Default Ports
- **API Server** : 9000
- **Network TCP** : 9001  
- **Network UDP** : 9002

## Architecture

```
src/
├── handlers.rs          # HTTP request handlers
├── middleware.rs        # Authentication and CORS
├── models.rs           # Data structures and types
├── server.rs           # Main server configuration
├── websocket.rs        # WebSocket connection manager
└── lib.rs              # Library entry point
```

## WebSocket Events

### Outgoing Events
```json
{
  "type": "network_update",
  "timestamp": "2025-01-06T12:00:00Z",
  "data": { /* network stats */ }
}

{
  "type": "block_created", 
  "timestamp": "2025-01-06T12:00:00Z",
  "data": { /* block info */ }
}
```

## Development

### Build for Development
```bash
cargo build
cargo run
```

### Build for Production
```bash
cargo build --release
cargo run --release
```

### Run Tests
```bash
cargo test
```

### Code Formatting
```bash
cargo fmt
cargo clippy
```

## Integration

### With EcoBlock Dashboard
The API is designed to work seamlessly with the EcoBlock dashboard:
```javascript
// Connect to API
const api = axios.create({
  baseURL: 'http://localhost:9000/api'
});

// WebSocket connection
const ws = new WebSocket('ws://localhost:9000/api/ws');
```

### Network Node Integration
The API integrates directly with EcoBlock network nodes for real-time data access.

## Performance

- **High Throughput** : Handles thousands of concurrent connections
- **Low Latency** : Sub-millisecond response times for cached data
- **Memory Efficient** : Optimized for minimal resource usage
- **Scalable** : Horizontal scaling ready

## Dependencies

### Core Dependencies
- `axum` - High-performance web framework
- `tokio` - Async runtime
- `serde` - Serialization framework
- `uuid` - Unique identifier generation

### EcoBlock Dependencies
- `ecoblock-network` - Network node integration
- `ecoblock-core` - Core blockchain functionality
- `ecoblock-crypto` - Cryptographic operations
- `ecoblock-storage` - Data persistence

## Contributing

1. Fork the project
2. Create a feature branch (`git checkout -b feature/new-feature`)
3. Commit your changes (`git commit -am 'Add new feature'`)
4. Push to the branch (`git push origin feature/new-feature`)
5. Create a Pull Request

## License

This project is part of the **EcoBlock Network** ecosystem.

---

**Built with Rust for maximum performance and reliability**
