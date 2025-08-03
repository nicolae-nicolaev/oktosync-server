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
    if let Ok(config) = config::load_config() {
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
    } else {
        log::error!("❌ Could not load config. Shutting down.");
    };
    Ok(())
}
