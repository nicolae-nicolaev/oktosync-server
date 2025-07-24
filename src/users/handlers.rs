use crate::users::{NewUser, User, errors::UserRegisterError, service::*};

use oktosync_server::AppState;

use axum::{Json, debug_handler, extract::State, http::StatusCode, response::IntoResponse};
use serde_json::json;

#[debug_handler]
pub async fn register_user(
    State(state): State<AppState>,
    Json(payload): Json<NewUser>,
) -> impl IntoResponse {
    let user = User {
        id: None,
        username: payload.username,
        email: payload.email,
        public_key: payload.public_key,
    };

    match add_user(&user, &state.db).await {
        Ok(_) => {
            let response = json!({
                "status": "success"
            });

            (StatusCode::CREATED, Json(response))
        }
        Err(err) => match err {
            UserRegisterError::UsernameTaken => {
                let response = json!({"status": "failure", "error": "Username already taken."});
                (StatusCode::BAD_REQUEST, Json(response))
            }
            UserRegisterError::EmailTaken => {
                let response = json!({"status": "failure", "error": "Email already taken."});
                (StatusCode::BAD_REQUEST, Json(response))
            }
            UserRegisterError::InvalidData(message) => {
                let response =
                    json!({"status": "failure", "error": format!("Invalid data: {message}")});
                (StatusCode::BAD_REQUEST, Json(response))
            }
            UserRegisterError::DbError(_) => {
                let response = json!({"status": "failure", "error": "Database operation failed."});
                (StatusCode::INTERNAL_SERVER_ERROR, Json(response))
            }
        },
    }
}
