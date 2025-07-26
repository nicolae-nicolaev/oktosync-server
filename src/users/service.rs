use validator::Validate;

use crate::users::User;
use crate::users::errors::UserRegisterError;

pub async fn add_user(user: &User, pool: &sqlx::PgPool) -> Result<(), UserRegisterError> {
    if let Err(e) = user.validate() {
        return Err(UserRegisterError::InvalidData(format!(
            "Validation error: {e}"
        )));
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
