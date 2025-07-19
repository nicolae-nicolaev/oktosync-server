use serde::Deserialize;

pub struct User {
    pub id: Option<i64>,
    pub username: String,
    pub email: String,
    pub public_key: String,
}

#[derive(Debug, Deserialize)]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub public_key: String,
}
