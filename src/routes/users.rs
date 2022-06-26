use mongodb::{Client, options::ClientOptions};
use mongodb::bson::doc;
use rocket::futures::TryStreamExt;
use rocket::serde::json::Json;

use crate::models::user::User;
use crate::utilities::result::Result;

#[get("/users")]
pub async fn get_users() -> Result<Json<Vec<User>>> {
    let mut client_options = ClientOptions::parse(dotenv!("TESTING_URL")).await?;
    client_options.app_name = Some("Sprint".to_string());

    let client = Client::with_options(client_options)?;
    let database = client.database("sprint-testing");
    let users = database
        .collection::<User>("users")
        .find(None, None)
        .await?;
    let users = users.try_collect().await?;
    Ok(Json(users))
}

#[post("/users", format = "application/json", data = "<user>")]
pub async fn post_user(user: User) -> Result<Json<User>> {
    let mut client_options = ClientOptions::parse(dotenv!("TESTING_URL")).await?;
    client_options.app_name = Some("Sprint".to_string());

    let client = Client::with_options(client_options)?;
    let database = client.database("sprint-testing");

    database.collection("users").insert_one(user.clone(), None).await?;

    Ok(Json(user))
}
