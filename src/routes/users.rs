use anyhow::Context;
use mongodb::bson::doc;
use rocket::http::Status;
use rocket::serde::json::Json;

use crate::models::user::User;
use crate::utilities::client::CLIENT_RUNTIME;

#[get("/users")]
pub fn get_users() -> Result<Json<Vec<User>>, Status> {
    let (client, rt) = &*CLIENT_RUNTIME;
    let result = rt.block_on(async {
        let database = client.database("sprint-testing");
        User::read(None, &database)
            .await
            .map(Json)
            .map_err(|_| Status::InternalServerError)
    })?;
    Ok(result)
}

#[get("/users/<username>")]
pub fn get_user(username: String) -> Result<Json<User>, Status> {
    let (client, rt) = &*CLIENT_RUNTIME;
    let result = rt.block_on(async {
        let database = client.database("sprint-testing");
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
    })?;
    Ok(result)
}

#[post("/users", format = "application/json", data = "<user>")]
pub fn post_user(user: User) -> Result<Json<User>, Status> {
    let (client, rt) = &*CLIENT_RUNTIME;
    let result = rt.block_on(async {
        let database = client.database("sprint-testing");
        User::create(user, &database)
            .await
            .map(Json)
            .map_err(|_| Status::InternalServerError)
    })?;
    Ok(result)
}

#[put("/users/<username>", format = "application/json", data = "<user>")]
pub fn put_user(username: String, user: User) -> Result<Json<User>, Status> {
    let (client, rt) = &*CLIENT_RUNTIME;
    let result = rt.block_on(async {
        let database = client.database("sprint-testing");
        User::update(username, user, &database)
            .await
            .map(Json)
            .map_err(|_| Status::InternalServerError)
    })?;
    Ok(result)
}

#[delete("/users/<username>", format = "application/json")]
pub fn delete_user(username: String) -> Result<(), Status> {
    let (client, rt) = &*CLIENT_RUNTIME;
    let result = rt.block_on(async {
        let database = client.database("sprint-testing");
        User::delete(username, &database)
            .await
            .map_err(|_| Status::InternalServerError)
    })?;
    Ok(result)
}
