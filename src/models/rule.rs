use super::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Rule {
    #[serde(rename = "from")]
    from: i64,

    #[serde(rename = "price")]
    price: f64,
}
