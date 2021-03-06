use rocket::{Data, Request};
use rocket::data::{FromData, Outcome, ToByteUnit};
use rocket::http::{ContentType, Status};
use rocket::outcome::Outcome::{Failure, Forward, Success};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct User {
    name: String,
    username: String,
    email: String,
    password: String,
    roles: Vec<Role>,
}

#[derive(Debug)]
pub enum Error {
    TooLarge,
    Io(anyhow::Error),
}

#[rocket::async_trait]
impl<'r> FromData<'r> for User {
    type Error = Error;

    async fn from_data(req: &'r Request<'_>, data: Data<'r>) -> Outcome<'r, Self> {
        use Error::*;
        let content_type = ContentType::new("application", "json");
        if req.content_type() != Some(&content_type) {
            return Forward(data)
        }

        let limit = req.limits().get("user").unwrap_or_else(|| 1000.bytes());

        let string = match data.open(limit).into_string().await {
            Ok(string) if string.is_complete() => string.into_inner(),
            Ok(_) => return Failure((Status::PayloadTooLarge, TooLarge)),
            Err(err) => return Failure((Status::InternalServerError, Io(err.into())))
        };

        return match serde_json::from_str::<User>(&*string) {
            Ok(user) => Success(user),
            Err(err) => Failure((Status::InternalServerError, Io(err.into())))
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Role {
    name: String,
    permissions: Permissions,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Permissions {
    read: Vec<String>,
    create: Vec<String>,
    update: Vec<String>,
    delete: Vec<String>,
}
