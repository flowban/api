use jwt_simple::prelude::*;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket_dyn_templates::handlebars::JsonValue;
use serde_json::json;
use crate::models::user::User;
use crate::utilities::client::CLIENT_RUNTIME;

#[derive(Serialize, Deserialize)]
pub struct Credentials {
    username: String,
    password: String
}

#[post("/login", data = "<credentials>")]
pub fn login(credentials: Json<Credentials>) ->  Result<Json<JsonValue>, Status> {
    let (client, rt) = &*CLIENT_RUNTIME;
    let result = rt.block_on(async {
        let database = client.database("sprint-testing");
        let username = credentials.username.to_string();
        let password = credentials.password.to_string();

        match User::exists(username, password, &database).await {
            Some(_) => {
                let key = HS256Key::generate();
                let claims = Claims::create(Duration::from_hours(2));
                match key.authenticate(claims) {
                    Ok(token) => Ok(Json(json!({ "success": true, "token": token }))),
                    Err(_) => Err(Status::InternalServerError)
                }
            },
            None => Err(Status::NotFound)
        }
    })?;
    Ok(result)
}