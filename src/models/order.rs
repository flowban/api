use super::{
    checkout::Checkout, client::Client, item::ItemTrait, order_status::OrderStatus,
    payment_method::PaymentMethod,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Order {
    #[serde(rename = "identifier")]
    identifier: String,

    #[serde(rename = "createdDate")]
    created_date: String,

    #[serde(rename = "deliveryDate")]
    delivery_date: String,

    #[serde(rename = "status")]
    status: OrderStatus,

    #[serde(rename = "client")]
    client: Client,

    #[serde(rename = "items")]
    items: Vec<Box<dyn ItemTrait>>,

    #[serde(rename = "checkout")]
    checkout: Checkout,

    #[serde(rename = "paymentMethod")]
    payment_method: PaymentMethod,
}
