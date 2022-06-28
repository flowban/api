use anyhow::Context;
use mongodb::bson::doc;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;
use crate::AppState;

use crate::models::user::User;

#[get("/users")]
    pub async fn get_users(state: &State<AppState>) -> Result<Json<Vec<User>>, Status> {
    let database = state.client.database("sprint-testing");
    User::read(None, &database)
        .await
        .map(Json)
        .map_err(|_| Status::InternalServerError)
}

#[get("/users/<username>")]
pub async fn get_user(username: String, state: &State<AppState>) -> Result<Json<User>, Status> {
    let database = state.client.database("sprint-testing");
    User::read(Some(username), &database)
        .await
        .map(|users| {
            Json(
                users
                    .get(0)
                    .with_context(|| "Failed to get user")
                    .unwrap()
                    .to_owned(),
            )
        })
        .map_err(|_| Status::InternalServerError)
}

#[post("/users", format = "application/json", data = "<user>")]
pub async fn post_user(user: User, state: &State<AppState>) -> Result<Json<User>, Status> {
    let database = state.client.database("sprint-testing");
    User::create(user, &database)
        .await
        .map(Json)
        .map_err(|_| Status::InternalServerError)
}

#[put("/users/<username>", format = "application/json", data = "<user>")]
pub async fn put_user(username: String, user: User, state: &State<AppState>) -> Result<Json<User>, Status> {
    let database = state.client.database("sprint-testing");
    User::update(username, user, &database)
        .await
        .map(Json)
        .map_err(|_| Status::InternalServerError)
}

#[delete("/users/<username>", format = "application/json")]
pub async fn delete_user(username: String, state: &State<AppState>) -> Result<(), Status> {
    let database = state.client.database("sprint-testing");
    User::delete(username, &database)
        .await
        .map_err(|_| Status::InternalServerError)
}
