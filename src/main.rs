mod api;
mod data;
mod repository;

#[macro_use] 
extern crate rocket;

use api::user_api::create_user;
use repository::mongodb_repo::MongoRepo;

#[launch]
fn rocket() -> _ {
    rocket::build()
    .manage(MongoRepo::init())
    .mount("/", routes![create_user])
}