use axum::{
    routing::{get, post},
    Json, Router,
};
use geo::Point;
use geozero::wkb;
use serde::{Deserialize, Serialize};

pub fn router() -> Router {
    Router::new()
        .route("/", get(get_latest_caches))
        .route("/new", post(post_new_cache))
        .route("/edit", post(edit_cache).delete(delete_cache))
}

async fn post_new_cache(Json(cache): Json<NewCache>) {}

async fn get_latest_caches() {}

async fn edit_cache() {}

async fn delete_cache() {}

#[derive(Serialize, Deserialize, sqlx::Type, Debug)]
pub enum CacheStatus {
    Archived,
    Maintenance,
    Active,
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

trait Saveable {
    fn save_to_db() {}
}

impl<'a> Saveable for NewCache {}
