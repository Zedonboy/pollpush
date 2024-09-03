// use stock_market_service::*;
use tokio;
use reqwest;
use warp::Filter;
use std::net::SocketAddr;

use crate::{models, rest_api, stock_data, websocket};

#[tokio::test]
async fn test_rest_api_integration() {
    // Start the server
    let stock_data = stock_data::initialize_stock_data();
    let (tx, _) = tokio::sync::broadcast::channel(100);
    let routes = rest_api::stock_routes(stock_data.clone())
        .or(warp::path("ws")
            .and(warp::ws())
            .and(warp::any().map(move || stock_data.clone()))
            .and(warp::any().map(move || tx.clone()))
            .and_then(websocket::ws_handler));

    let (addr, server) = warp::serve(routes).bind_ephemeral(([127, 0, 0, 1], 0));
    tokio::spawn(server);

    // Test GET /api/stocks
    let client = reqwest::Client::new();
    let response = client.get(format!("http://{}/api/stocks", addr))
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(response.status(), 200);

    let stocks: Vec<models::StockQuote> = response.json().await.expect("Failed to parse JSON");
    assert!(!stocks.is_empty());

    // Test GET /api/stock/AAPL
    let response = client.get(format!("http://{}/api/stock/AAPL", addr))
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(response.status(), 200);

    let stock: models::StockQuote = response.json().await.expect("Failed to parse JSON");
    assert_eq!(stock.symbol, "AAPL");
}