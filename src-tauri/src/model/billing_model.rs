use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BillingSubscription {
    pub id: String,
    pub user_id: String,
    pub plan: String,
    pub status: String,
    pub cancelled_at: Option<String>,
    pub cancel_at_period_end: String,
    pub current_period_end: String,
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BillingData {
    pub subscriptions: Vec<BillingSubscription>,
}
