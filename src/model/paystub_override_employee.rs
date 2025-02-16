
use serde::{Serialize, Deserialize};
use super::PaystubOverrideEmployeeAddress;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaystubOverrideEmployee {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<PaystubOverrideEmployeeAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl std::fmt::Display for PaystubOverrideEmployee {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}