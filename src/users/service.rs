use crate::users::User;
use crate::users::errors::UserRegisterError;

pub async fn add_user(user: &User, pool: &sqlx::PgPool) -> Result<(), UserRegisterError> {
    if !validator::ValidateEmail::validate_email(&user.email) {
        return Err(UserRegisterError::InvalidData(format!(
            "Invalid email: {}",
            user.email
        )));
    }

    if user.username.is_empty() {
        return Err(UserRegisterError::InvalidData(
            "No username provided.".to_string(),
        ));
    }

    if user.public_key.is_empty() {
        return Err(UserRegisterError::InvalidData(
            "No public key provided.".to_string(),
        ));
    }

    let query = "INSERT INTO users (username, email, public_key) VALUES ($1, $2, $3)";

    let result = sqlx::query(query)
        .bind(&user.username)
        .bind(&user.email)
        .bind(&user.public_key)
        .execute(pool)
        .await;

    match result {
        Ok(_) => Ok(()),
        Err(sqlx::Error::Database(db_err)) if db_err.constraint() == Some("users_username_key") => {
            Err(UserRegisterError::UsernameTaken)
        }
        Err(sqlx::Error::Database(db_err)) if db_err.constraint() == Some("users_email_key") => {
            Err(UserRegisterError::EmailTaken)
        }
        Err(e) => Err(UserRegisterError::from(e)),
    }
}
