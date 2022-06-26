use core::panicking::panic;
use super::*;
use anyhow::Result;
use mongodb::bson::doc;
use mongodb::Database;
use rocket::futures::TryStreamExt;

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct User {
    name: String,
    pub(crate) username: String,
    email: String,
    pub(crate) password: String,
    roles: Vec<Role>,
}

impl User {
    pub async fn read(username: Option<String>, database: &Database) -> Result<Vec<User>> {
        let filter = match username {
            None => None,
            Some(username) => Some(doc! {"username": username}),
        };
        let users = database
            .collection::<User>("users")
            .find(filter, None)
            .await?;
        let users = users.try_collect().await?;
        Ok(users)
    }
    pub async fn create(user: Self, database: &Database) -> Result<User> {
        database
            .collection("users")
            .insert_one(user.clone(), None)
            .await?;
        Ok(user)
    }
    pub async fn update(username: String, user: User, database: &Database) -> Result<User> {
        database
            .collection("users")
            .update_one(doc! {"username": username}, user.clone(), None)
            .await?;
        Ok(user)
    }
    pub async fn delete(username: String, database: &Database) -> Result<()> {
        database
            .collection("users")
            .delete_one(doc! {"username": username}, None)
            .await?;
        Ok(())
    }
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
            return Forward(data);
        }

        let limit = req.limits().get("user").unwrap_or_else(|| 1000.bytes());

        let string = match data.open(limit).into_string().await {
            Ok(string) if string.is_complete() => string.into_inner(),
            Ok(_) => return Failure((Status::PayloadTooLarge, TooLarge)),
            Err(err) => return Failure((Status::InternalServerError, Io(err.into()))),
        };

        return match serde_json::from_str::<User>(&*string) {
            Ok(user) => Success(user),
            Err(err) => Failure((Status::InternalServerError, Io(err.into()))),
        };
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
