use hyperliquid::client::HyperLiquidClient;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = HyperLiquidClient::new();

    // // Get all mid prices
    // let all_mids = client.get_all_mids().await?;
    // println!("All mid prices: {:#?}", all_mids);

    // // Get user open orders
    // let user_open_orders = client.get_open_orders("0x6f90d048a511626ba5a6425db17f377826df336a").await?;
    // println!("User open orders: {:#?}", user_open_orders);

    // let frontend_open_orders = client.get_frontend_open_orders("0x6f90d048a511626ba5a6425db17f377826df336a").await?;
    // println!("Frontend open orders: {:#?}", frontend_open_orders);

    // let user_fills = client.get_user_fills("0xcA1c5C696f27888E1D496466Ad99DF6e589FD634").await?;
    // println!("User fills: {:#?}", user_fills);
    
    // let user_fills_by_time = client.get_user_fills_by_time("0xcA1c5C696f27888E1D496466Ad99DF6e589FD634", 1719859200, Default::default(), true).await?;
    // println!("User fills by time: {:#?}", user_fills_by_time);

    // let user_rate_limit = client.get_user_rate_limit("0xcA1c5C696f27888E1D496466Ad99DF6e589FD634").await?;
    // println!("User rate limit: {:#?}", user_rate_limit);

    // let order_status = client.get_order_status("0xcA1c5C696f27888E1D496466Ad99DF6e589FD634", 95975679641).await?;
    // println!("Order status: {:#?}", order_status);

    // let l2_book = client.get_l2_book("BTC").await?;
    // println!("L2 book: {:#?}", l2_book);

    let candle_snapshot = client.get_candle_snapshot("BTC", "1m", 1751540616000, 1751540716000).await?;
    println!("Candle snapshot: {:#?}", candle_snapshot);

    // let historical_orders = client.get_historical_orders("0xcA1c5C696f27888E1D496466Ad99DF6e589FD634").await?;
    // println!("Historical orders: {:#?}", historical_orders);
    
    // let user_twap_slice_fills = client.get_user_twap_slice_fills("0x325edd95bb016c36027ce3b9f7595af7094a9564").await?;
    // println!("User twap slice fills: {:#?}", user_twap_slice_fills);

    // let sub_accounts = client.get_sub_accounts("0x325edd95bb016c36027ce3b9f7595af7094a9564").await?;
    // println!("Sub accounts: {:#?}", sub_accounts);

    // let vault_details = client.get_vault_details("0x07fd993f0fa3a185f7207adccd29f7a87404689d").await?;
    // println!("Vault details: {:#?}", vault_details);
    
    // let user_vault_equities = client.get_user_vault_equities("0x2b804617c6f63c040377e95bb276811747006f4b").await?;
    // println!("User vault equities: {:#?}", user_vault_equities);

    // let user_role = client.get_user_role("0x2b804617c6f63c040377e95bb276811747006f4b").await?;
    // println!("User role: {:#?}", user_role);

    // let portfolio = client.get_portfolio("0xcA1c5C696f27888E1D496466Ad99DF6e589FD634").await?;
    // println!("Portfolio: {:#?}", portfolio);

    // let referral_info = client.get_referral_info("0xcA1c5C696f27888E1D496466Ad99DF6e589FD634").await?;
    // println!("Referral info: {:#?}", referral_info);

    // let user_fees = client.get_user_fees("0xcA1c5C696f27888E1D496466Ad99DF6e589FD634").await?;
    // println!("User fees: {:#?}", user_fees);

    // let delegations = client.get_delegations("0xcA1c5C696f27888E1D496466Ad99DF6e589FD634").await?;
    // println!("Delegations: {:#?}", delegations);

    // let delegator_summary = client.get_delegator_summary("0xcA1c5C696f27888E1D496466Ad99DF6e589FD634").await?;
    // println!("Delegator summary: {:#?}", delegator_summary);

    // let delegator_history = client.get_delegator_history("0xcA1c5C696f27888E1D496466Ad99DF6e589FD634").await?;
    // println!("Delegator history: {:#?}", delegator_history);

    // let delegator_rewards = client.get_delegator_rewards("0xcA1c5C696f27888E1D496466Ad99DF6e589FD634").await?;
    // println!("Delegator rewards: {:#?}", delegator_rewards);

    // let perp_dexs = client.get_perp_dexs().await?;
    // println!("Perp dexs: {:#?}", perp_dexs);
    
    Ok(())
}