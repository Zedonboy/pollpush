use futures_util::{SinkExt, StreamExt};
use tokio::sync::broadcast;
use tokio_tungstenite::connect_async;
use warp::Filter;

use crate::websocket::ws_handler;

#[tokio::test]
async fn test_websocket_connection() {
    let stock_data = crate::stock_data::initialize_stock_data();
    let (tx, _) = broadcast::channel(100);

    let routes = warp::path("ws")
        .and(warp::ws())
        .and(warp::any().map(move || stock_data.clone()))
        .and(warp::any().map(move || tx.clone()))
        .and_then(ws_handler);

    let (addr, server) = warp::serve(routes).bind_ephemeral(([127, 0, 0, 1], 0));
    tokio::spawn(server);

    let url = format!("ws://{}/ws", addr);
    println!("{}", url);
    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
    let (mut write, mut read) = ws_stream.split();

    // Optional: Send a message to the server
    write
        .send(tokio_tungstenite::tungstenite::Message::Text(
            "Hello".to_string(),
        ))
        .await
        .expect("Failed to send message");

    let msg = read
        .next()
        .await
        .expect("Failed to read message")
        .expect("WebSocket error");
    assert!(msg.is_text());

    println!("Received message: {:?}", msg);

    // You can add more assertions here based on the expected message content
}
