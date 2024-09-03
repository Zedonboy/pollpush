use tokio_test::block_on;

use crate::stock_data::{initialize_stock_data, update_stock_prices};

#[test]
    fn test_initialize_stock_data() {
        let stock_data = initialize_stock_data();
        let data = block_on(stock_data.read());
        assert!(data.contains_key("AAPL"));
        assert!(data.contains_key("GOOGL"));
    }

    #[tokio::test]
    async fn test_update_stock_prices() {
        let stock_data = initialize_stock_data();
        let initial_price = {
            let data = stock_data.read().await;
            data.get("AAPL").unwrap().price
        };

        // Run the update function for a short time
        let update_handle = tokio::spawn(update_stock_prices(stock_data.clone()));
        tokio::time::sleep(tokio::time::Duration::from_secs(6)).await;
        update_handle.abort();

        let final_price = {
            let data = stock_data.read().await;
            data.get("AAPL").unwrap().price
        };

        assert_ne!(initial_price, final_price);
    }