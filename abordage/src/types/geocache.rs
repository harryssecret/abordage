use crate::PGDb;
use geozero::wkb;
use rocket::fairing::AdHoc;
use rocket::response::status::Created;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket_db_pools::{sqlx, Connection};

#[derive(Serialize, Deserialize, sqlx::Type, Debug)]
enum CacheStatus {
    Archived,
    Maintenance,
    Active,
}

struct CacheRecord {
    location: wkb::Decode<geo_types::Geometry<f64>>,
}

#[derive(Serialize)]
struct Cache {
    id: i32,
    cache_name: String,
    location: Point,
    maintainer: String,
    difficulty: i16,
    archived_state: CacheStatus,
}

#[derive(Serialize, Deserialize)]
struct NewCache {
    cache_name: String,
    maintainer: String,
    location: Point,
    difficulty: i16,
}

impl Saveable for NewCache {}

trait Saveable {
    fn save_to_db() {}
}

type Result<T, E = rocket::response::Debug<sqlx::Error>> = std::result::Result<T, E>;

#[get("/")]
async fn retrieve_cache_list(mut pool: Connection<PGDb>) -> Result<Json<Vec<Cache>>> {
    let caches_list = sqlx::query_as!(Cache, r#"SELECT id, cache_name, maintainer, cache_location as "location!: _", difficulty, archived_state as "archived_state: CacheStatus" from caches"#)
    .fetch(&mut *pool)
    .map_ok(|r| Cache { id: r.id, cache_name: r.cache_name, maintainer: r.maintainer, location: r.location, difficulty: r.difficulty, archived_state: r.archived_state })
    .try_collect::<Vec<_>>()
    .await?;

    Ok(Json(caches_list))
}

#[post("/new", data = "<new_cache>")]
async fn insert_cache(
    mut pool: Connection<PGDb>,
    new_cache: Json<NewCache>,
) -> Result<Created<Json<NewCache>>> {
    sqlx::query_as!(NewCache,
        r#"INSERT INTO caches (cache_name, maintainer, cache_location, difficulty) VALUES ($1, $2, $3::geometry, $4);"#,
        new_cache.cache_name, new_cache.maintainer, wkb::Encode(new_cache.location) as _, new_cache.difficulty).execute(&mut *pool).await?;
    Ok(Created::new("/").body(new_cache))
}

#[get("/<id>")]
async fn retrieve_cache(mut pool: Connection<PGDb>, id: i32) -> Result<Json<Cache>> {
    let cache = sqlx::query_as!(Cache, r#"SELECT id, cache_name, maintainer, cache_location as "location! : _", difficulty, archived_state as "archived_state: _" from caches where id = $1"#, id)
        .fetch_one(&mut *pool)
        .await?;
    Ok(Json(cache))
}

#[put("/edit/<id>")]
async fn edit_cache(id: i32) {
    todo!()
}

#[delete("/delete/<id>")]
async fn delete_cache(id: i32) {
    todo!()
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
