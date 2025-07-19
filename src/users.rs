#![allow(dead_code)]

use std::error::Error;

pub struct User {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub public_key: String,
    pub active: bool,
}

pub mod user_service {
    use super::*;

    pub async fn add_user(user: &User, pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
        let query = "INSERT INTO users (username, email, public_key) VALUES ($1, $2, $3)";

        sqlx::query(query)
            .bind(&user.username)
            .bind(&user.email)
            .bind(&user.public_key)
            .execute(pool)
            .await?;

        Ok(())
    }

    pub async fn update_user(user: &User, pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
        let query = "UPDATE users SET username = $1, email = $2 WHERE id = $3";

        sqlx::query(query)
            .bind(&user.username)
            .bind(&user.email)
            .bind(user.id)
            .execute(pool)
            .await?;

        Ok(())
    }
}

pub mod handlers {
    use axum::{Json, http::StatusCode, response::IntoResponse};
    use serde_json::json;

    use super::*;

    pub struct NewUser {
        username: String,
        email: String,
        public_key: String,
    }

    pub async fn register_user(Json(payload): Json<NewUser>) -> impl IntoResponse {
        let user = User {
            id: 1,
            username: payload.username,
            email: payload.email,
            public_key: payload.public_key,
            active: true,
        };

        let response = json!({
            "status": "success"
        });

        (StatusCode::CREATED, Json(response))
    }
}
