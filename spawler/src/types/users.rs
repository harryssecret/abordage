use crate::PGDb;
use rocket::fairing::AdHoc;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket_db_pools::{sqlx, Connection};

#[derive(Serialize, Deserialize)]
struct ResponseResult {
    success: bool,
}

#[derive(Serialize, Deserialize)]
struct NewPirate {
    pub username: String,
    password: String,
}

struct Pirate {
    pub username: String,
    password: String,
    display_name: String,
    points: i32,
    is_admin: bool,
}

#[derive(Serialize, Deserialize)]
struct PirateInfos {
    id: i32,
    username: String,
    points: i32,
}

type Result<T, E = rocket::response::Debug<sqlx::Error>> = std::result::Result<T, E>;

#[post("/", data = "<user_json>")]
async fn add_user(
    mut pool: Connection<PGDb>,
    user_json: Json<NewPirate>,
) -> Result<Json<ResponseResult>> {
    sqlx::query!(
        "INSERT INTO users (username, pirate_password) VALUES ($1, $2)",
        user_json.username,
        user_json.password
    )
    .execute(&mut *pool)
    .await?;
    Ok(Json(ResponseResult { success: true }))
}

#[derive(Serialize)]
struct PirateProfile {
    username: String,
    display_name: Option<String>,
    gamepoints: i32,
}

#[get("/<id>")]
async fn retrieve_profile(mut pool: Connection<PGDb>, id: i32) -> Result<Json<PirateProfile>> {
    let user_profile = sqlx::query_as!(
        PirateProfile,
        "SELECT username, display_name, gamepoints FROM users WHERE id = $1",
        id
    )
    .fetch_one(&mut *pool)
    .await?;
    Ok(Json(user_profile))
}

pub fn register() -> AdHoc {
    AdHoc::on_ignite("Users route ignite", |rocket| async {
        rocket.mount("/users", routes![add_user, retrieve_profile])
    })
}
