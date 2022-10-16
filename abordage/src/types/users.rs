use serde::{Deserialize, Serialize};

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
