#[macro_use]
extern crate dotenv_codegen;
#[macro_use]
extern crate rocket;

use dotenv::dotenv;
use rocket_anyhow::Result;

use routes::users::get_users;

mod models;
mod routes;
mod utilities;

#[get("/")]
fn index() -> &'static str {
    "Sprint API"
}

#[rocket::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let _ = rocket::build().mount("/", routes![index, get_users]).launch().await?;
    Ok(())
}
