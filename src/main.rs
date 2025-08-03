mod config;
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
    match config::load_config() {
        Ok(config) => {
            let url = &config.database.database_url;
            let pool = sqlx::postgres::PgPool::connect(url).await?;

            sqlx::migrate!("./migrations").run(&pool).await?;

            let state = AppState { db: pool };

            let app = Router::new()
                .route("/", get(handlers::index))
                .route("/register", post(users::handlers::register_user))
                .route("/upload", post(handlers::upload))
                .with_state(state);

            let address = format!("{}:{}", config.server.host, config.server.port);

            let listener = tokio::net::TcpListener::bind(address).await?;
            axum::serve(listener, app).await?;
        }
        Err(err) => {
            return Err(format!("❌ Could not load config: {err}").into());
        }
    }

    Ok(())
}
