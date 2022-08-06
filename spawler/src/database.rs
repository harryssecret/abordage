use diesel::{Connection, PgConnection};
use dotenv::dotenv;
use std::env;

pub fn etablish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("Your database URL doesn't seem to be set.");
    PgConnection::establish(&database_url).expect("Impossible to connect to the current URL.")
}
