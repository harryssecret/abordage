use rocket::{
    fairing::{self, AdHoc},
    Build, Rocket,
};
use rocket_db_pools::{sqlx, Database};

#[macro_use]
extern crate rocket;

pub mod types;

#[derive(Database)]
#[database("abordage")]
struct PGDb(sqlx::PgPool);

#[launch]
async fn rocket() -> _ {
    rocket::build()
        .attach(PGDb::init())
        .attach(AdHoc::try_on_ignite("Migrations", run_migrations))
        .attach(types::geocache::register())
        .attach(types::users::register())
}

async fn run_migrations(rocket: Rocket<Build>) -> fairing::Result {
    match PGDb::fetch(&rocket) {
        Some(db) => match sqlx::migrate!("./migrations").run(&**db).await {
            Ok(_) => Ok(rocket),
            Err(e) => {
                error!("Impossible to initialize the database : {}", e);
                Err(rocket)
            }
        },
        None => Err(rocket),
    }
}
