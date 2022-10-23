use std::sync::Arc;

use axum::{
    extract::{Json, Path},
    routing::{get, post},
    Extension, Router,
};
use serde::{Deserialize, Serialize};

use crate::AppContext;

pub fn router() -> Router {
    Router::new()
        .route("/:user_id", get(get_user))
        .route("/new", post(create_user))
        .route("/edit/:user_id", post(edit_user).delete(delete_user))
}

async fn create_user(Json(user): Json<NewPirate>, Extension(ctx): Extension<Arc<AppContext>>) {
    sqlx::query!(
        r#"INSERT INTO users (username, pirate_password) VALUES ($1, $2)"#,
        user.username,
        user.password
    )
    .execute(&ctx.db)
    .await
    .expect("Impossible to insert new user.");
}

async fn edit_user(
    Json(user_infos): Json<PirateEditInfos>,
    Extension(ctx): Extension<Arc<AppContext>>,
) {
    sqlx::query!(
        r#"UPDATE users SET display_name = $1, pirate_password = $2"#,
        user_infos.display_name,
        user_infos.password
    )
    .execute(&ctx.db)
    .await
    .expect("Impossible to edit user.");
}

async fn delete_user(Path(user_id): Path<i32>, Extension(ctx): Extension<Arc<AppContext>>) {
    sqlx::query!(r#"DELETE FROM users WHERE id = $1"#, user_id)
        .execute(&ctx.db)
        .await
        .expect("Impossible to delete user.");
}

async fn get_user(Path(user_id): Path<i32>, Extension(ctx): Extension<Arc<AppContext>>) {
    sqlx::query_as!(
        PirateProfile,
        r#"SELECT username, display_name, gamepoints FROM users WHERE id = $1;"#,
        user_id
    )
    .fetch_one(&ctx.db)
    .await
    .expect("Impossible to get user.");
}

#[derive(Serialize, Deserialize)]
struct NewPirate {
    pub username: String,
    pub password: String,
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

#[derive(Serialize)]
struct PirateProfile {
    username: String,
    display_name: Option<String>,
    gamepoints: i32,
}

#[derive(Serialize, Deserialize)]
struct PirateEditInfos {
    display_name: Option<String>,
    password: Option<String>,
}
