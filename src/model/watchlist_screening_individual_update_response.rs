
use serde::{Serialize, Deserialize};
use super::{WatchlistScreeningAuditTrail, WatchlistScreeningSearchTerms};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchlistScreeningIndividualUpdateResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignee: Option<String>,
    pub audit_trail: WatchlistScreeningAuditTrail,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_user_id: Option<String>,
    pub id: String,
    pub request_id: String,
    pub search_terms: WatchlistScreeningSearchTerms,
    pub status: String,
}
impl std::fmt::Display for WatchlistScreeningIndividualUpdateResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}