
use serde::{Serialize, Deserialize};
use super::WatchlistScreeningAuditTrail;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndividualWatchlistProgram {
    pub audit_trail: WatchlistScreeningAuditTrail,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub id: String,
    pub is_archived: bool,
    pub is_rescanning_enabled: bool,
    pub lists_enabled: Vec<String>,
    pub name: String,
    pub name_sensitivity: String,
}
impl std::fmt::Display for IndividualWatchlistProgram {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}