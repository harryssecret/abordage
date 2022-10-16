use geo::Point;
use geozero::wkb;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, sqlx::Type, Debug)]
enum CacheStatus {
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
