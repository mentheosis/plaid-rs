
use serde::{Serialize, Deserialize};
use super::EarningsBreakdownCanonicalDescription;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EarningsBreakdown {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canonical_description: Option<EarningsBreakdownCanonicalDescription>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hours: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iso_currency_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unofficial_currency_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ytd_amount: Option<f64>,
}
impl std::fmt::Display for EarningsBreakdown {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}