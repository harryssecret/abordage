use rocket::serde::json::{json, Json};
use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Pirate {
    username: String,
    points: i32,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Coordinate {
    longitude: (i32, f32),
    latitude: (i32, f32),
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Cache {
    name: String,
    coordinates: (Coordinate, Coordinate),
    difficulty: i8,
    pirate: Pirate,
    archived: bool,
}

impl Cache {
    fn new(
        name: String,
        coordinateX: Coordinate,
        coordinateY: Coordinate,
        difficulty: i8,
        pirate: Pirate,
        archived: bool,
    ) -> Cache {
        Cache {
            name,
            coordinates: (coordinateX, coordinateY),
            difficulty,
            pirate,
            archived,
        }
    }
}

#[post("/cache/new")]
fn insert_cache() -> Json<Cache> {}

#[get("/cache/<id>")]
fn cache(id: i32) {}

pub fn register() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("JSON", |rocket| async {
        rocket.mount("/caches", routes![])
    })
}
