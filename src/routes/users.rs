use mongodb::{Client, options::ClientOptions};
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
    let users = database.collection::<User>("users");
    let cursor = users.find(None, None).await?;
    let users = cursor.try_collect().await.unwrap_or_else(|_| vec![]);
    Ok(Json(users))
}
