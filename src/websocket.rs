use std::sync::Arc;

use crate::stock_data::StockData;
use futures_util::{SinkExt, StreamExt};
use log::{error, info};
use tokio::sync::{broadcast, Mutex};
use warp::ws::{Message, WebSocket};
use serde_json::json;

pub async fn handle_ws_client(ws: WebSocket, stock_data: StockData, mut rx: broadcast::Receiver<()>) {
    let (mut ws_tx, mut ws_rx) = ws.split();

    let ws_tx = Arc::new(Mutex::new(ws_tx));
    
    let ws_for_broadcaster = ws_tx.clone();
    tokio::spawn(async move {
        while rx.recv().await.is_ok() {
            let data = stock_data.read().await;
            let stocks: Vec<_> = data.values().cloned().collect();
            if let Ok(json) = serde_json::to_string(&stocks) {
                let mut sender = ws_for_broadcaster.lock().await;
                if let Err(e) = sender.send(Message::text(json)).await {
                    error!("WebSocket send error: {}", e);
                    break;
                }
            }
        }
    });

    let ws_for_ws_client = ws_tx.clone();

    while let Some(result) = ws_rx.next().await {
       
        match result {
            Ok(msg) => {
                if let Ok(text) = msg.to_str() {
                    info!("Received message from client: {}", text);
                    let retun_mssg = Message::text("Hi");
                    let mut  sender = ws_for_ws_client.lock().await;
                    if let Err(e) = sender.send(retun_mssg).await{
                        error!("WebSocket send error: {}", e);
                        break;
                    }
                    // Handle client messages if needed
                }
            }
            Err(e) => {
                error!("WebSocket error: {}", e);
                break;
            }
        }
    }

    info!("WebSocket client disconnected");
}

pub async fn ws_handler(ws: warp::ws::Ws, stock_data: StockData, tx: broadcast::Sender<()>) -> Result<impl warp::Reply, warp::Rejection> {
    println!("Handle");
    Ok(ws.on_upgrade(move |socket| handle_ws_client(socket, stock_data, tx.subscribe())))
}