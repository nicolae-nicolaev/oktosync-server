mod config;
mod users;
mod web;

use oktosync_server::AppState;
use sqlx::postgres::PgPoolOptions;
use tokio::signal;

use std::error::Error;

use web::handlers;

use axum::{
    Router,
    routing::{get, post},
};

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

async fn run() -> Result<(), Box<dyn Error>> {
    let settings = config::load_config()?;
    let db_url = config::resolve_database_url(&settings)?;

    let pg_pool = PgPoolOptions::new()
        .max_connections(15)
        .connect(&db_url)
        .await?;

    sqlx::migrate!("./migrations").run(&pg_pool).await?;

    let state = AppState { db: pg_pool };
    let app = Router::new()
        .route("/", get(handlers::index))
        .route("/health", get(handlers::health))
        .route("/register", post(users::handlers::register_user))
        .route("/upload", post(handlers::upload))
        .with_state(state);

    let address = format!(
        "{}:{}",
        settings.server.host.expect("No server host provided."),
        settings.server.port.expect("No server port provided.")
    );

    let listener = tokio::net::TcpListener::bind(&address).await?;

    println!("🚀 oktosync-server listening on http://{address}");

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("Failed to install shutdown signal.");
    };

    #[cfg(unix)]
    let terminate = async {
        use tokio::signal::unix::{SignalKind, signal};
        let mut sigterm =
            signal(SignalKind::terminate()).expect("Failed to install SIGTERM handler.");
        sigterm.recv().await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {_ = ctrl_c => {}, _ = terminate => {}}
}
