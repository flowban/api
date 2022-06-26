use jwt::token::Signed;
use sha2::Digest;
use rocket::outcome::Outcome;
use rocket::request::{self, FromRequest, Request};
use sha2::Sha256;

pub extern crate jwt;
pub extern crate rustc_serialize;

use self::jwt::{Header, RegisteredClaims, Token};

pub struct ApiKey(pub String);

pub fn read_token(key: &str) -> Result<String, String> {
    let token =
        Token::<Header, RegisteredClaims, Signed>::parse(key).map_err(|_| "Unable to parse key".to_string())?;
    if token.verify(b"secret_key", Sha256::new()) {
        token.claims.sub.ok_or("Claims not valid".to_string())
    } else {
        Err("Token not valid".to_string())
    }
}

impl<'a, 'r> FromRequest<'a> for ApiKey {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<ApiKey, ()> {
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
