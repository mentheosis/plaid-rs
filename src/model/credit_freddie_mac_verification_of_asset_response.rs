
use serde::{Serialize, Deserialize};
use super::CreditFreddieMacAssets;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditFreddieMacVerificationOfAssetResponse {
    #[serde(rename = "ASSETS")]
    pub assets: CreditFreddieMacAssets,
}
impl std::fmt::Display for CreditFreddieMacVerificationOfAssetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}