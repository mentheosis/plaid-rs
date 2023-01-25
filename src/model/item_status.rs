
use serde::{Serialize, Deserialize};
use super::{ItemStatusInvestments, ItemStatusLastWebhook, ItemStatusTransactions};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ItemStatus {
    pub investments: Option<ItemStatusInvestments>,
    pub last_webhook: Option<ItemStatusLastWebhook>,
    pub transactions: Option<ItemStatusTransactions>,
}
impl std::fmt::Display for ItemStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}