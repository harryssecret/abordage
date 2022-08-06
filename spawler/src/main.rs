#[macro_use]
extern crate rocket;
pub mod database;
pub mod geocache;
pub mod users;

#[launch]
fn rocket() -> _ {
    geocache::register();
    rocket::build().mount("/", routes![])
}
