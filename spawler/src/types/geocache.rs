use crate::PGDb;
use rocket::fairing::AdHoc;
use rocket::futures::TryStreamExt;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket_db_pools::sqlx;
use rocket_db_pools::Connection;
use std::fmt;

#[derive(Serialize, Deserialize, Debug, sqlx::Type)]
#[sqlx(type_name = "point")]

struct Point {
    x: f32,
    y: f32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Serialize)]
struct Cache {
    id: i32,
    cache_name: String,
    maintainer_name: String,
    coordinate_x: Point,
    coordinate_y: Point,
    difficulty: i16,
    archived_state: CacheStatus,
}

#[derive(Deserialize)]
struct NewCache {
    cache_name: String,
    maintainer: String,
    coordinate_x: Point,
    coordinate_y: Point,
    difficulty: i16,
    archived_state: CacheStatus,
}

#[derive(Serialize, Deserialize, sqlx::Type, Debug)]
#[sqlx(type_name = "cache_status", rename_all = "snake_case")]
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

impl fmt::Display for NewCache {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}, {}, {}, {}, {}, {}",
            self.cache_name,
            self.coordinate_x,
            self.coordinate_y,
            self.difficulty,
            self.maintainer,
            self.archived_state
        )
    }
}

#[get("/")]
async fn retrieve_cache_list(mut pool: Connection<PGDb>) {
    let caches_list = sqlx::query!(r#"SELECT id, cache_name, maintainer, coordinate_x as "coordinate_x: Point", coordinate_y as "coordinate_y: Point", difficulty, archived_state as "archived_state: CacheStatus" from caches"#)
    .fetch(&mut *pool).map_ok(|r| Json(Cache { id: r.id, cache_name: r.cache_name, maintainer_name: r.maintainer, coordinate_x: r.coordinate_x, coordinate_y: r.coordinate_y, difficulty: r.difficulty, archived_state: r.archived_state })).try_collect::<Vec<_>>().await;
}

#[post("/new", data = "<new_cache>")]
async fn insert_cache(mut pool: Connection<PGDb>, new_cache: Json<NewCache>) {
    let request = format!("INSERT INTO caches VALUES ({})", *new_cache);
    let new_cache = sqlx::query(&request).fetch_one(&mut *pool).await;
}

#[get("/<id>")]
async fn retrieve_cache(mut pool: Connection<PGDb>, id: i32) {
    sqlx::query!(
        r#"SELECT cache_name, coordinate_x as "coordinate_x: Point", coordinate_y as "coordinate_y: Point", difficulty, archived_state as "archived_state: CacheStatus" FROM caches where id = $1 "#,
        id
    ).fetch_one(&mut *pool).await;
    unimplemented!()
}

#[put("/edit/<id>")]
async fn edit_cache(id: i32) {
    unimplemented!()
}

#[delete("/delete/<id>")]
async fn delete_cache(id: i32) {
    unimplemented!()
}

pub fn register() -> AdHoc {
    AdHoc::on_ignite("Caches routes ignite", |rocket| async {
        rocket.mount(
            "/caches",
            routes![
                retrieve_cache_list,
                insert_cache,
                retrieve_cache,
                edit_cache,
                delete_cache
            ],
        )
    })
}
