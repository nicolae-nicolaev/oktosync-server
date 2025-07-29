mod users;
mod web;

use oktosync_server::AppState;

use std::error::Error;

use web::handlers;

use axum::{
    Router,
    routing::{get, post},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "postgres://oktosync:password@localhost:5432/oktosync";
    let pool = sqlx::postgres::PgPool::connect(url).await?;
    sqlx::migrate!("./migrations").run(&pool).await?;

    let state = AppState { db: pool };

    let app = Router::new()
        .route("/", get(handlers::index))
        .route("/register", post(users::handlers::register_user))
        .route("/upload", post(handlers::upload))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}
