use std::sync::Arc;

use axum::{
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
        .route("/new", post(post_new_cache))
        .route("/edit", post(edit_cache).delete(delete_cache))
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

async fn edit_cache(Extension(ctx): Extension<Arc<AppContext>>) {}

async fn delete_cache(Extension(ctx): Extension<Arc<AppContext>>) {}

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
