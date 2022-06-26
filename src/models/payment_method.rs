use super::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentMethod {
    #[serde(rename = "id")]
    id: String,

    #[serde(rename = "displayName")]
    display_name: String,
}
