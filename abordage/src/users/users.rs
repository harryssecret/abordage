use std::sync::Arc;

use axum::{extract::Path, Extension, Form};
use serde::{Deserialize, Serialize};

use crate::AppContext;

async fn create_user(Form(user): Form<NewPirate>, Extension(ctx): Extension<Arc<AppContext>>) {
    sqlx::query!(
        r#"INSERT INTO users (username, pirate_password) VALUES ($1, $2)"#,
        user.username,
        user.password
    )
    .execute(&ctx.db)
    .await
    .expect("Impossible to insert new user.");
}

async fn edit_user(Extension(ctx): Extension<Arc<AppContext>>) {}

async fn delete_user(Extension(ctx): Extension<Arc<AppContext>>) {}

async fn get_user(Path(user_id): Path<i32>, Extension(ctx): Extension<Arc<AppContext>>) {}

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

#[derive(Serialize)]
struct PirateProfile {
    username: String,
    display_name: Option<String>,
    gamepoints: i32,
}
