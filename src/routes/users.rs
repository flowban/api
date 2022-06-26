use crate::utilities::client::CLIENT;
use anyhow::Context;
use mongodb::bson::doc;
use mongodb::Database;
use rocket::http::Status;
use rocket::serde::json::Json;

use crate::models::user::User;

static DATABASE: Database = CLIENT.database("sprint-testing");

#[get("/users")]
pub async fn get_users() -> Result<Json<Vec<User>>, Status> {
    User::read(None, &DATABASE)
        .await
        .map(Json)
        .map_err(|_| Status::InternalServerError)
}

#[get("/users/<username>")]
pub async fn get_user(username: String) -> Result<Json<User>, Status> {
    User::read(Some(username), &DATABASE)
        .await
        .map(|users| {
            Json(
                users
                    .get(0)
                    .with_context(|| "Failed to get user")?
                    .to_owned(),
            )
        })
        .map_err(|_| Status::InternalServerError)
}

#[post("/users", format = "application/json", data = "<user>")]
pub async fn post_user(user: User) -> Result<Json<User>, Status> {
    User::create(user, &DATABASE)
        .await
        .map(Json)
        .map_err(|_| Status::InternalServerError)
}

#[put("/users/<username>", format = "application/json", data = "<user>")]
pub async fn put_user(username: String, user: User) -> Result<Json<User>, Status> {
    User::update(username, user, &DATABASE)
        .await
        .map(Json)
        .map_err(|_| Status::InternalServerError)
}

#[delete("/users/<username>", format = "application/json")]
pub async fn delete_user(username: String) -> Result<(), Status> {
    User::delete(username, &DATABASE)
        .await
        .map_err(|_| Status::InternalServerError)
}
