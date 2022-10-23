use std::sync::Arc;

use axum::{
    extract::Path,
    routing::{get, post},
    Extension, Json, Router,
};
use geo::Point;
use geozero::wkb;
use serde::{Deserialize, Serialize};

use crate::AppContext;

pub fn router() -> Router {
    Router::new()
        .route("/", get(get_latest_caches))
        .route("/:cache_id", get(get_cache))
        .route("/new", post(post_new_cache))
        .route("/edit/:cache_id", post(edit_cache).delete(delete_cache))
}

async fn post_new_cache(Json(cache): Json<NewCache>, Extension(ctx): Extension<Arc<AppContext>>) {
    let location: geo_types::Geometry<f64> = cache.location.into();
    sqlx::query!(r#"INSERT INTO caches (cache_name, maintainer, cache_location, difficulty) VALUES ($1, $2, $3::geometry, $4)"#, cache.cache_name, cache.maintainer, wkb::Encode(location) as _, cache.difficulty)
        .execute(&ctx.db)
        .await
        .expect("Impossible to insert new cache.");
}

async fn get_latest_caches(Extension(ctx): Extension<Arc<AppContext>>) {
    sqlx::query_as!(
        ShortCacheRecord,
        r#"SELECT id, cache_name, cache_location as "location!: _" FROM caches"#
    )
    .fetch_all(&ctx.db)
    .await
    .expect("Impossible to get latest caches.");
}

async fn edit_cache(
    Json(cache_infos): Json<CacheEditInfos>,
    Path(cache_id): Path<i32>,
    Extension(ctx): Extension<Arc<AppContext>>,
) {
    sqlx::query!(
        r#"UPDATE caches SET cache_name = $1, difficulty = $2 WHERE id = $3"#,
        cache_infos.cache_name,
        cache_infos.difficulty,
        cache_id
    )
    .execute(&ctx.db)
    .await
    .expect("Impossible to edit cache.");
}

async fn delete_cache(Path(cache_id): Path<i32>, Extension(ctx): Extension<Arc<AppContext>>) {}

async fn get_cache(Path(cache_id): Path<i32>, Extension(ctx): Extension<Arc<AppContext>>) {
    sqlx::query_as!(
        ShortCacheRecord,
        r#"SELECT id, cache_name, cache_location as "location!: _" FROM caches WHERE id = $1"#,
        cache_id
    )
    .fetch_one(&ctx.db)
    .await
    .expect("Impossible to get cache.");
}

#[derive(Serialize, Deserialize, sqlx::Type, Debug)]
pub enum CacheStatus {
    Archived,
    Maintenance,
    Active,
}

struct ShortCacheRecord {
    id: i32,
    cache_name: String,
    location: wkb::Decode<geo_types::Geometry<f64>>,
}

struct CacheRecord {
    id: i32,
    cache_name: String,
    maintainer: String,
    difficulty: i16,
    archived_state: CacheStatus,
    location: wkb::Decode<geo_types::Geometry<f64>>,
}

#[derive(Serialize)]
pub struct Cache {
    cache_name: String,
    location: Point,
    maintainer: String,
    difficulty: i8,
    archived_state: CacheStatus,
}

impl Cache {
    pub fn new(
        cache_name: String,
        location: Point,
        maintainer: String,
        difficulty: i8,
        archived_state: CacheStatus,
    ) -> Cache {
        Cache {
            cache_name,
            location,
            maintainer,
            difficulty,
            archived_state,
        }
    }
}

#[derive(Serialize, Deserialize)]
struct NewCache {
    cache_name: String,
    maintainer: String,
    location: Point,
    difficulty: i16,
}

#[derive(Serialize, Deserialize)]
struct CacheEditInfos {
    cache_name: Option<String>,
    difficulty: Option<i16>,
}
