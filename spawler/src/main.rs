#[macro_use]
extern crate rocket;

pub mod database;
mod geocache;
mod users;

#[launch]
fn rocket() -> _ {
    geocache::register();
    rocket::build().mount("/", routes![])
}
