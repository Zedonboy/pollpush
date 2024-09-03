mod models;
mod rest_api;
mod websocket;
mod stock_data;

use log::info;
use std::net::SocketAddr;
use tokio::sync::broadcast;
use warp::Filter;

#[cfg(test)]
mod tests;

#[tokio::main]
async fn main() {
    env_logger::init();

    let stock_data = stock_data::initialize_stock_data();
    let (tx, _) = broadcast::channel(100);

    let dx = stock_data.clone();
    let routes = rest_api::stock_routes(stock_data.clone())
        .or(warp::path("ws")
            .and(warp::ws())
            .and(warp::any().map(move || dx.clone()))
            .and(warp::any().map(move || tx.clone()))
            .and_then(websocket::ws_handler))
        .recover(rest_api::handle_rejection);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3030));
    
    info!("Server starting on {}", addr);

    let server = warp::serve(routes).run(addr);
    let stock_updater = stock_data::update_stock_prices(stock_data.clone());
    // let notifier = async move {
    //     loop {
    //         tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    //         let _ = tx.send(());
    //     }
    // };

    tokio::join!(server, stock_updater);
}