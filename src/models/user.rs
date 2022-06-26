use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
struct Oid {
    #[serde(rename = "$oid")]
    oid: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id")]
    _id: Oid,

    #[serde(rename = "name")]
    name: String,

    #[serde(rename = "username")]
    username: String,

    #[serde(rename = "email")]
    email: String,

    #[serde(rename = "password")]
    password: String,

    #[serde(rename = "role")]
    role: Vec<Role>,

    #[serde(rename = "area")]
    area: Area,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Area {
    #[serde(rename = "identifier")]
    identifier: String,

    #[serde(rename = "displayName")]
    display_name: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Role {
    #[serde(rename = "id")]
    id: String,

    #[serde(rename = "displayName")]
    display_name: String,

    #[serde(rename = "permissions")]
    permissions: Vec<Permission>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Permission {
    #[serde(rename = "id")]
    id: String,

    #[serde(rename = "displayName")]
    display_name: String,
}
