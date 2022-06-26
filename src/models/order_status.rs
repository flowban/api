use super::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderStatus {
    #[serde(rename = "id")]
    id: String,

    #[serde(rename = "displayName")]
    display_name: String,
}
