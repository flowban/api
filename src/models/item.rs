use super::*;
use super::{product::Product, service::Service};

#[typetag::serde(tag = "type")]
pub trait ItemTrait {}

#[typetag::serde]
impl ItemTrait for Product {}
#[typetag::serde]
impl ItemTrait for Service {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Item<T: ItemTrait> {
    #[serde(rename = "quantity")]
    quantity: i64,

    #[serde(rename = "item")]
    item: T,

    #[serde(rename = "observations")]
    observations: String,
}
