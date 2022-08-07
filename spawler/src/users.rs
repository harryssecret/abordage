use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Pirate {
    username: String,
    password: String,
    points: i32,
    is_admin: bool,
}

impl Pirate {
    fn new(username: String, password: String) -> Pirate {
        Pirate {
            username,
            password,
            points: 0,
            is_admin: false,
        }
    }
}
