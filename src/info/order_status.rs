use serde::{Deserialize, Serialize};

use crate::{client::HyperLiquidClient, errors::validate_ethereum_address, types::Order};

#[derive(Serialize)]
struct OrderStatusRequest {
    #[serde(rename = "type")]
    request_type: String,
    user: String,
    oid: u64,
}

#[derive(Debug, Deserialize)]
pub struct OrderStatusResponse {
    pub status: String,
    pub order: OrderStatus,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderStatus {
    pub order: Order,
    pub status: String,
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



