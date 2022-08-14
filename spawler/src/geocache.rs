use crate::users::Pirate;
use crate::PGDb;
use rocket::fairing::AdHoc;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket::{fairing, Build, Rocket};
use rocket_db_pools::{sqlx, Connection, Database};
use std::fmt;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]

struct Point(f32, f32);

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Cache {
    name: String,
    pirate: Pirate,
    coordinate_x: Point,
    coordinate_y: Point,
    difficulty: i8,
    archived: CacheStatus,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
enum CacheStatus {
    Archived,
    Maintenance,
    Active,
}

impl fmt::Display for CacheStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Archived => write!(f, "archived"),
            Self::Maintenance => write!(f, "maintenance"),
            Self::Active => write!(f, "active"),
        }
    }
}

impl Cache {
    fn new(
        name: String,
        coordinate_x: Point,
        coordinate_y: Point,
        difficulty: i8,
        pirate: Pirate,
        archived: CacheStatus,
    ) -> Result<Cache, &'static str> {
        if difficulty < 0 || difficulty > 5 {
            return Err("Invalid difficulty");
        }
        Ok(Cache {
            name,
            coordinate_x,
            coordinate_y,
            difficulty,
            pirate,
            archived,
        })
    }

    async fn to_db(&self, mut pool: Connection<PGDb>) {
        let request = format!("INSERT INTO caches VALUES ({})", self);
        let new_cache = sqlx::query(&request).fetch_one(&mut *pool).await;
    }
}

impl fmt::Display for Cache {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}, {}, {}, {}, {}, {}",
            self.name,
            self.coordinate_x,
            self.coordinate_y,
            self.difficulty,
            self.pirate.username,
            self.archived
        )
    }
}

#[get("/")]
async fn retrieve_cache_list() {
    unimplemented!()
}

#[post("/new", data = "<new_cache>")]
async fn insert_cache(pool: Connection<PGDb>, new_cache: Json<Cache>) {
    new_cache.to_db(pool).await;
}

#[get("/<id>")]
fn retrieve_cache(id: i32) {
    unimplemented!()
}

#[put("/edit/<id>")]
fn edit_cache(id: i32) {
    unimplemented!()
}

#[delete("/delete/<id>")]
fn delete_cache(id: i32) {
    unimplemented!()
}

pub fn register() -> AdHoc {
    AdHoc::on_ignite("Cache ignite", |rocket| async {
        rocket.attach(PGDb::init()).mount(
            "/cache",
            routes![insert_cache, retrieve_cache, edit_cache, delete_cache],
        )
    })
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
