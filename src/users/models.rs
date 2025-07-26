use serde::Deserialize;

use validator::Validate;

#[derive(Validate)]
pub struct User {
    pub id: Option<i64>,
    #[validate(length(min = 6))]
    pub username: String,
    #[validate(email)]
    pub email: String,
    pub public_key: String,
}

#[derive(Debug, Deserialize)]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub public_key: String,
}
