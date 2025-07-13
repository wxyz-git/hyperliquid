use serde::{Deserialize, Serialize};
use crate::client::HyperLiquidClient;

#[derive(Serialize)]
struct OrderStatusRequest {
    #[serde(rename = "type")]
    request_type: String,
    user: String,
    oid: u64,
}

#[derive(Deserialize, Debug)]
pub struct OrderStatusResponse {
    pub status: String,
    pub order: OrderStatus,
}

#[derive(Deserialize, Debug)]
pub struct OrderStatus {
    pub order: Order,
    pub status: String,
    #[serde(rename = "statusTimestamp")]
    pub status_timestamp: u64,
}

// Order struct is now imported from crate::types

impl HyperLiquidClient {
    pub async fn get_order_status(&self, user: &str, oid: u64) -> anyhow::Result<OrderStatusResponse> {
        validate_ethereum_address(user)?;
        
        let request_body = OrderStatusRequest {
            request_type: "orderStatus".to_string(),
            user: user.to_string(),
            oid: oid,
        };
    
        self.make_custom_request(&request_body).await
    }   
}



