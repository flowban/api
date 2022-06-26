use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::rule::Rule;

#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
    #[serde(rename = "id")]
    id: String,

    #[serde(rename = "displayName")]
    display_name: String,

    #[serde(rename = "price")]
    price: f64,

    #[serde(rename = "rules")]
    rules: Vec<Rule>,

    #[serde(rename = "categories")]
    categories: Vec<Category>,

    #[serde(rename = "specification")]
    specification: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Category {
    #[serde(rename = "id")]
    id: String,

    #[serde(rename = "displayName")]
    display_name: String,
}
