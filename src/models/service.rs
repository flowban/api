use super::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Service {
    #[serde(rename = "id")]
    id: String,

    #[serde(rename = "displayName")]
    display_name: String,

    #[serde(rename = "description")]
    description: String,

    #[serde(rename = "recurrence")]
    recurrence: Recurrence,

    #[serde(rename = "specification")]
    specification: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Recurrence {
    #[serde(rename = "identifier")]
    identifier: String,

    #[serde(rename = "displayName")]
    display_name: String,
}
