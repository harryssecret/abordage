use rocket_db_pools::{sqlx, Database};

#[macro_use]
extern crate rocket;

mod geocache;
mod users;

#[derive(Database)]
#[database("abordage")]
struct PGDb(sqlx::PgPool);

#[launch]
async fn rocket() -> _ {
    rocket::build().attach(geocache::register())
}
