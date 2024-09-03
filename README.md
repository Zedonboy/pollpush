# Rust Poll-Push Server

## Overview

This Rust Poll-Push Server is a high-performance, concurrent stock market data service that demonstrates both poll-based (REST API) and push-based (WebSocket) data synchronization models. It's built using modern Rust async programming techniques and showcases best practices for building scalable network applications.

## Features

- **Dual Synchronization Models**: 
  - Poll-based REST API for on-demand data retrieval
  - Push-based WebSocket for real-time updates
- **Concurrent Stock Data Management**: Utilizes `Arc<RwLock<>>` for thread-safe access to shared state
- **Real-time Stock Price Updates**: Simulates periodic stock price changes
- **Scalable Architecture**: Designed to handle multiple concurrent clients efficiently
- **Comprehensive Logging**: Detailed logging for debugging and monitoring
- **Docker Support**: Easy deployment using Docker

## Technology Stack

- Rust 1.68+
- Tokio for async runtime
- Warp for web server framework
- Serde for serialization/deserialization
- env_logger for logging

## Prerequisites

- Rust 1.68 or higher
- Docker (for containerized deployment)

## Setup and Installation

1. Clone the repository:
   ```
   git clone https://github.com/yourusername/rust-pollpush-server.git
   cd rust-pollpush-server
   ```

2. Build the project:
   ```
   cargo build --release
   ```

3. Run the server:
   ```
   cargo run --release
   ```

## Docker Deployment

1. Build the Docker image:
   ```
   docker build -t rust-pollpush-server .
   ```

2. Run the container:
   ```
   docker run -p 3030:3030 rust-pollpush-server
   ```

## Usage

### REST API

- Get all stocks: `GET /api/stocks`
- Get a specific stock: `GET /api/stock/{symbol}`

Example:
```
curl http://localhost:3030/api/stocks
curl http://localhost:3030/api/stock/AAPL
```

### WebSocket

Connect to the WebSocket endpoint at `ws://localhost:3030/ws`

Available commands:
- `get_all_stocks`: Retrieves all stock data
- `get_stock:{symbol}`: Retrieves data for a specific stock

Example using wscat:
```
wscat -c ws://localhost:3030/ws
Connected (press CTRL+C to quit)
> get_all_stocks
< [{"symbol":"AAPL","price":150.23,"timestamp":"2023-09-02T12:34:56Z"},...]
> get_stock:GOOGL
< {"symbol":"GOOGL","price":2805.12,"timestamp":"2023-09-02T12:34:57Z"}
```

## Configuration

- Default port: 3030 (configurable in `main.rs`)
- Log level: Controlled by `RUST_LOG` environment variable (default: `info`)

## Testing

Run the test suite:
```
cargo test
```

## Logging

- View logs in Docker:
  ```
  docker logs -f rust-pollpush-server
  ```

- Adjust log level:
  ```
  docker run -e RUST_LOG=debug rust-pollpush-server
  ```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Rust Community for excellent documentation and crates
- Contributors and maintainers of Tokio, Warp, and other used libraries