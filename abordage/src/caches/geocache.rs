use std::sync::Arc;

use axum::{
    extract::Path,
    routing::{get, post},
    Extension, Json, Router,
};
use geo_types::Point;
use geozero::wkb;
use serde::{Deserialize, Serialize};

use crate::AppContext;

pub fn router() -> Router {
    Router::new()
        .route("/", get(get_latest_caches))
        .route("/:cache_id", get(get_geocache))
        .route("/new", post(post_new_cache))
        .route("/edit/:cache_id", post(edit_cache).delete(delete_geocache))
}

async fn post_new_cache(
    Json(geocache): Json<NewGeocache>,
    Extension(ctx): Extension<Arc<AppContext>>,
) {
    let location: geo_types::Geometry<f64> = geocache.location.into();
    sqlx::query!(r#"INSERT INTO caches (cache_name, maintainer, cache_location, difficulty) VALUES ($1, $2, $3::geometry, $4)"#, geocache.cache_name, geocache.maintainer, wkb::Encode(location) as _, geocache.difficulty)
        .execute(&ctx.db)
        .await
        .expect("Impossible to insert new cache.");
}

async fn get_latest_caches(Extension(ctx): Extension<Arc<AppContext>>) {
    sqlx::query_as!(
        ShortGeocacheRecord,
        r#"SELECT id, cache_name, cache_location as "location!: _" FROM caches"#
    )
    .fetch_all(&ctx.db)
    .await
    .expect("Impossible to get latest caches.");
}

async fn edit_cache(
    Json(geocache_infos): Json<GeocacheEditInfos>,
    Path(geocache_id): Path<i32>,
    Extension(ctx): Extension<Arc<AppContext>>,
) {
    sqlx::query!(
        r#"UPDATE caches SET cache_name = $1, difficulty = $2 WHERE id = $3"#,
        geocache_infos.cache_name,
        geocache_infos.difficulty,
        geocache_id
    )
    .execute(&ctx.db)
    .await
    .expect("Impossible to edit cache.");
}

async fn delete_geocache(Path(geocache_id): Path<i32>, Extension(ctx): Extension<Arc<AppContext>>) {
}

async fn get_geocache(Path(geocache_id): Path<i32>, Extension(ctx): Extension<Arc<AppContext>>) {
    sqlx::query_as!(
        ShortGeocacheRecord,
        r#"SELECT id, cache_name, cache_location as "location!: _" FROM caches WHERE id = $1"#,
        geocache_id
    )
    .fetch_one(&ctx.db)
    .await
    .expect("Impossible to get cache.");
}

#[derive(Serialize, Deserialize, sqlx::Type, Debug)]
pub enum GeocacheStatus {
    Archived,
    Maintenance,
    Active,
}

struct ShortGeocacheRecord {
    id: i32,
    cache_name: String,
    location: wkb::Decode<geo_types::Geometry<f64>>,
}

struct GeocacheRecord {
    id: i32,
    cache_name: String,
    maintainer: String,
    difficulty: i16,
    archived_state: GeocacheStatus,
    location: wkb::Decode<geo_types::Geometry<f64>>,
}

#[derive(Serialize)]
pub struct Geocache {
    cache_name: String,
    location: Point,
    maintainer: String,
    difficulty: i8,
    archived_state: GeocacheStatus,
}

impl Geocache {
    pub fn new(
        cache_name: String,
        location: Point,
        maintainer: String,
        difficulty: i8,
        archived_state: GeocacheStatus,
    ) -> Geocache {
        Geocache {
            cache_name,
            location,
            maintainer,
            difficulty,
            archived_state,
        }
    }
}

#[derive(Serialize, Deserialize)]
struct NewGeocache {
    cache_name: String,
    maintainer: String,
    location: geo_types::Point,
    difficulty: i16,
}

#[derive(Serialize, Deserialize)]
struct GeocacheEditInfos {
    cache_name: Option<String>,
    difficulty: Option<i16>,
}
