mod model;

#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;

use dotenv::dotenv;
use std::env;


#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
