use crate::users::{NewUser, User, service::*};

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
        Err(err) => {
            eprintln!("Failed to insert user: {err}");

            let response = json!({
                "status": "failure"
            });

            (StatusCode::BAD_REQUEST, Json(response))
        }
    }
}
