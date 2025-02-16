
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AssetReportFilterResponse {
    pub asset_report_id: String,
    pub asset_report_token: String,
    pub request_id: String,
}
impl std::fmt::Display for AssetReportFilterResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}