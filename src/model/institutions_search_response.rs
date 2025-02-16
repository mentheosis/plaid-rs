
use serde::{Serialize, Deserialize};
use super::Institution;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InstitutionsSearchResponse {
    pub institutions: Vec<Institution>,
    pub request_id: String,
}
impl std::fmt::Display for InstitutionsSearchResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}