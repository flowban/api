use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Checkout {
    #[serde(rename = "subTotal")]
    sub_total: i64,

    #[serde(rename = "total")]
    total: i64,
}
