use std::{collections::HashMap, sync::Arc};

use chrono::Utc;
use mockall::mock;
use tokio::sync::RwLock;
use warp::test::request;

use crate::{models::StockQuote, rest_api::{get_all_stocks, get_stock_quote}, stock_data::StockData};

mock! {
    StockData {}
    impl Clone for StockData {
        fn clone(&self) -> Self;
    }
}

fn create_mock_stock_data() -> StockData {
    let mut data = HashMap::new();
    data.insert(
        "AAPL".to_string(),
        StockQuote {
            symbol: "AAPL".to_string(),
            price: 150.0,
            timestamp: Utc::now().timestamp() as u64,
        },
    );
    Arc::new(RwLock::new(data))
}

#[tokio::test]
async fn test_get_stock_quote() {
    let stock_data = create_mock_stock_data();
    
    let routes = get_stock_quote(stock_data);

    let response = request()
        .method("GET")
        .path("/api/stock/AAPL")
        .reply(&routes)
        .await;

    assert_eq!(response.status(), 200);
    // You would add more assertions here based on the expected response
}

#[tokio::test]
async fn test_get_all_stocks() {
    let stock_data = create_mock_stock_data();
    
    let routes = get_all_stocks(stock_data);

    let response = request()
        .method("GET")
        .path("/api/stocks")
        .reply(&routes)
        .await;

    assert_eq!(response.status(), 200);
    // You would add more assertions here based on the expected response
}