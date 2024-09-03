use crate::models::StockQuote;
use chrono::Utc;
use rand::rngs::StdRng;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use rand::{Rng, SeedableRng};

pub type StockData = Arc<RwLock<HashMap<String, StockQuote>>>;

pub fn initialize_stock_data() -> StockData {
    let mut data = HashMap::new();
    data.insert("AAPL".to_string(), StockQuote { symbol: "AAPL".to_string(), price: 150.0, timestamp: Utc::now().timestamp() as u64 });
    data.insert("GOOGL".to_string(), StockQuote { symbol: "GOOGL".to_string(), price: 2800.0, timestamp: Utc::now().timestamp() as u64 });
    Arc::new(RwLock::new(data))
}

pub async fn update_stock_prices(stock_data: StockData) {
    let mut rng = StdRng::from_entropy();
    
    loop {
        {
            let mut data = stock_data.write().await;
            for quote in data.values_mut() {
                let change = rng.gen_range(-5.0..5.0);
                quote.price += change;
                quote.timestamp = Utc::now().timestamp() as u64;
            }
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    }
}