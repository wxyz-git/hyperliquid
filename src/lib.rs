pub mod client;
pub mod info {
    pub mod all_mids;
    pub mod candle_snapshot;
    pub mod delegations;
    pub mod delegator_history;
    pub mod delegator_rewards;
    pub mod delegator_summary;
    pub mod frontend_open_orders;
    pub mod historical_orders;
    pub mod l2_book;
    pub mod max_builder_fee;
    pub mod portfolio;
    pub mod referral;
    pub mod open_orders;
    pub mod order_status;
    pub mod sub_accounts;
    pub mod user_fees;
    pub mod user_fills;
    pub mod user_fills_by_time;
    pub mod user_rate_limit;
    pub mod user_role;
    pub mod user_twap_slice_fills;
    pub mod user_vault_equities;
    pub mod vault_details;
    pub mod perpetuals{
        pub mod perpetuals;
        pub mod perp_dexs;
        }
}