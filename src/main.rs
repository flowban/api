#[macro_use]
extern crate dotenv_codegen;
#[macro_use]
extern crate rocket;

use anyhow::Result;
use dotenv::dotenv;
use rocket::request::Request;
use rocket_dyn_templates::{context, Template};

use routes::users::{
    get_users,
    post_user,
};

mod models;
mod routes;
mod utilities;

#[get("/")]
fn index() -> Template {
    Template::render("home", context! {
        first_name: "Jane",
        last_name: "Doe"
    })
}

#[catch(500)]
fn internal_error() -> Template {
    Template::render("500", context! {})
}

#[get("/500")]
fn internal_errors() -> Template {
    Template::render("500", context! {})
}

#[catch(404)]
fn not_found(req: &Request) -> Template {
    Template::render("404", context! {
        path: req.uri()
    })
}

#[rocket::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let _ = rocket::build()
        .register("/", catchers![not_found, internal_error])
        .mount("/", routes![index, internal_errors])
        .mount("/api", routes![get_users, post_user])
        .attach(Template::fairing())
        .launch()
        .await?;
    Ok(())
}
