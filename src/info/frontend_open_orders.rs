use crate::client::HyperLiquidClient;
use crate::errors::validate_ethereum_address;
use crate::types::Order;

pub type FrontendOpenOrdersResponse = Order;

impl HyperLiquidClient {
    pub async fn get_frontend_open_orders(&self, user: &str) -> anyhow::Result<Vec<FrontendOpenOrdersResponse>> {
        validate_ethereum_address(user)?;
        self.make_user_request("frontendOpenOrders", user).await
    }
}