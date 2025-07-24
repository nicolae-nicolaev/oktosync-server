pub enum UserRegisterError {
    UsernameTaken,
    EmailTaken,
    InvalidData(String),
    DbError(sqlx::Error),
}

impl From<sqlx::Error> for UserRegisterError {
    fn from(error: sqlx::Error) -> Self {
        UserRegisterError::DbError(error)
    }
}

pub enum UserUpdateError {
    UserNotFound,
    InvalidData(String),
    DbError(sqlx::Error),
}

impl From<sqlx::Error> for UserUpdateError {
    fn from(error: sqlx::Error) -> Self {
        UserUpdateError::DbError(error)
    }
}
