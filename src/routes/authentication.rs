use jwt_simple::prelude::*;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;
use rocket_dyn_templates::handlebars::JsonValue;
use serde_json::json;

use crate::AppState;
use crate::models::user::{Role, User};

#[derive(Serialize, Deserialize)]
pub struct Credentials {
    username: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
pub struct ClaimData {
    is_admin: bool,
}

#[post("/login", data = "<credentials>")]
pub async fn login(
    credentials: Json<Credentials>,
    state: &State<AppState>,
) -> Result<Json<JsonValue>, Status> {
    let database = state.client.database("sprint-testing");
    let username = credentials.username.to_string();
    let password = credentials.password.to_string();

    match User::exists(username, password, &database).await {
        Some(_) => {
            let key = HS256Key::generate().with_key_id(dotenv!("UUID"));
            let claims =
                Claims::with_custom_claims(ClaimData { is_admin: true }, Duration::from_hours(2))
                    .with_issuer("Flowban Inc.");
            match key.authenticate(claims) {
                Ok(token) => Ok(Json(json!({ "success": true, "token": token }))),
                Err(_) => Err(Status::InternalServerError),
            }
        }
        None => Err(Status::NotFound),
    }
}
