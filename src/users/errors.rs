pub enum UserRegistrationError {
    UsernameTaken,
    EmailTaken,
    InvalidData(String),
    DbError(sqlx::Error),
}

impl From<sqlx::Error> for UserRegistrationError {
    fn from(error: sqlx::Error) -> Self {
        UserRegistrationError::DbError(error)
    }
}

