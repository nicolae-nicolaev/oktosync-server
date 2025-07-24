use crate::users::User;
use crate::users::errors::UserRegistrationError;

use std::error::Error;

pub async fn add_user(user: &User, pool: &sqlx::PgPool) -> Result<(), UserRegistrationError> {
    if username_exists(&user.username, pool).await? {
        return Err(UserRegistrationError::UsernameTaken);
    }

    if email_exists(&user.email, pool).await? {
        return Err(UserRegistrationError::EmailTaken);
    }

    let query = "INSERT INTO users (username, email, public_key) VALUES ($1, $2, $3)";

    sqlx::query(query)
        .bind(&user.username)
        .bind(&user.email)
        .bind(&user.public_key)
        .execute(pool)
        .await
        .map_err(UserRegistrationError::from)?;

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

async fn username_exists(username: &str, pool: &sqlx::PgPool) -> Result<bool, sqlx::Error> {
    let query = "SELECT EXISTS(SELECT 1 FROM users WHERE username = $1)";
    let (exists,) = sqlx::query_as::<_, (bool,)>(query)
        .bind(username)
        .fetch_one(pool)
        .await?;

    Ok(exists)
}

async fn email_exists(email: &str, pool: &sqlx::PgPool) -> Result<bool, sqlx::Error> {
    let query = "SELECT EXISTS(SELECT 1 FROM users WHERE email = $1)";
    let (exists,) = sqlx::query_as::<_, (bool,)>(query)
        .bind(email)
        .fetch_one(pool)
        .await?;

    Ok(exists)
}
