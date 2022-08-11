use std::fmt;

use rocket::serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Pirate {
    pub username: String,
    password: String,
    points: i32,
    is_admin: bool,
}

impl Pirate {
    fn new(username: String, password: String) -> Pirate {
        Pirate {
            username,
            password,
            points: 0,
            is_admin: false,
        }
    }

    async fn to_db(self, conn: &Pool<Postgres>) {
        let query = format!("INSERT INTO users VALUES ({})", self);
        let new_user = sqlx::query(&query).execute(conn);
    }
}

impl fmt::Display for Pirate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}, {}, {}, {}",
            self.username, self.password, self.points, self.is_admin
        )
    }
}
