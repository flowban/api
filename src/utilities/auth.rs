use anyhow::Context;
use jwt_simple::prelude::*;
use rocket::outcome::Outcome;
use rocket::request::{self, FromRequest, Request};

pub struct ApiKey(pub String);

pub fn read_token(token: &str) -> anyhow::Result<String> {
    let key = HS256Key::generate();
    let claims = key.verify_token::<NoCustomClaims>(token, None)?;
    claims.subject.with_context(|| "Failed to get claims")
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiKey {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let keys: Vec<_> = request.headers().get("Authentication").collect();
        if keys.len() != 1 {
            return Outcome::Forward(());
        }
        match read_token(keys[0]) {
            Ok(claim) => Outcome::Success(ApiKey(claim)),
            Err(_) => Outcome::Forward(()),
        }
    }
}
