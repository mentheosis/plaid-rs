
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AssetReportGetRequestOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days_to_include: Option<i64>,
}
impl std::fmt::Display for AssetReportGetRequestOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}