use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Client {
    #[serde(rename = "id")]
    id: String,

    #[serde(rename = "displayName")]
    display_name: String,

    #[serde(rename = "address")]
    address: String,

    #[serde(rename = "cellular")]
    cellular: String,

    #[serde(rename = "office_number")]
    office_number: String,

    #[serde(rename = "business_name")]
    business_name: String,
}
