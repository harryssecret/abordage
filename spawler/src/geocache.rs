use crate::users::Pirate;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Coordinate(f32, f32);

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
        coordinate_x: Coordinate,
        coordinate_y: Coordinate,
        difficulty: i8,
        pirate: Pirate,
        archived: bool,
    ) -> Result<Cache, &'static str> {
        if difficulty < 0 || difficulty > 5 {
            return Err("Invalid difficulty");
        }
        Ok(Cache {
            name,
            coordinates: (coordinate_x, coordinate_y),
            difficulty,
            pirate,
            archived,
        })
    }
}

#[get("/")]
async fn retrieve_cache_list() {}

#[post("/new", data = "<new_cache>")]
fn insert_cache(new_cache: Json<Cache>) {
    unimplemented!()
}

#[get("/<id>")]
fn retrieve_cache(id: i32) {}

#[put("/edit/<id>")]
fn edit_cache(id: i32) {}

#[delete("/delete/<id>")]
fn delete_cache(id: i32) {}

pub fn register() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("JSON", |rocket| async {
        rocket.mount(
            "/cache",
            routes![insert_cache, retrieve_cache, edit_cache, delete_cache],
        )
    })
}
