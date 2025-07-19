use crate::users::User;

use std::error::Error;

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
