
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransactionsEnrichRequestOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_legacy_category: Option<bool>,
}
impl std::fmt::Display for TransactionsEnrichRequestOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}