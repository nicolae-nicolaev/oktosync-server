use std::path::Path;

use axum::{
    Router,
    body::Bytes,
    extract::Multipart,
    http::HeaderMap,
    routing::{get, post},
};
use clap::crate_version;
use tokio::{
    fs::{self, File},
    io::AsyncWriteExt,
};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index))
        .route("/upload", post(upload));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn index() -> String {
    format!("🐙 Welcome to OktoSync {}!", crate_version!())
}

async fn upload(headers: HeaderMap, mut multipart: Multipart) {
    let user_agent = get_header(&headers, "user-agent");
    let client_id = get_header(&headers, "x-client-id");

    println!("UserAgent: {user_agent}");
    println!("ClientID: {client_id}");

    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.file_name().unwrap().to_string();
        let data = field.bytes().await.unwrap();

        let path = format!("uploads/{client_id}/{name}"); // TODO: config

        println!("Length of `{name}` is {} bytes.", data.len());
        if let Err(e) = save_file(&path, data).await {
            eprintln!("Failed to save file: {e}");
        };
    }
}

async fn register_client() {}

async fn save_file(path: &str, data: Bytes) -> std::io::Result<()> {
    let path = Path::new(path);

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).await?;
    }

    let mut file = File::create(path).await?;

    file.write_all(&data).await?;
    Ok(())
}

fn get_header(headers: &HeaderMap, name: &str) -> String {
    headers
        .get(name)
        .and_then(|h| h.to_str().ok())
        .unwrap_or("unknown")
        .to_string()
}
