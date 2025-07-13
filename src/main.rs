use hyperliquid::client::HyperLiquidClient;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Create a new HyperLiquid client with default configuration
    let client = HyperLiquidClient::new();

    // Example: Get all mid prices for all trading pairs
    println!("Fetching all mid prices...");
    match client.get_all_mids().await {
        Ok(mid_prices) => {
            println!("✅ Successfully fetched {} trading pairs", mid_prices.0.len());
            if let Some(btc_price) = mid_prices.0.get("BTC") {
                println!("📈 BTC price: ${}", btc_price);
            }
        }
        Err(e) => {
            println!("❌ Failed to fetch mid prices: {}", e);
        }
    }

    // Example: Get L2 order book for BTC
    println!("\nFetching BTC order book...");
    match client.get_l2_book("BTC").await {
        Ok(order_book) => {
            println!("✅ BTC order book fetched");
            println!("📊 {} bids, {} asks", order_book.levels.len(), order_book.levels.len());
        }
        Err(e) => {
            println!("❌ Failed to fetch order book: {}", e);
        }
    }

    
    Ok(())
}