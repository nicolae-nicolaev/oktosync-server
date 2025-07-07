use axum::{
    Router,
    body::Bytes,
    extract::Multipart,
    routing::{get, post},
};
use tokio::{fs::File, io::AsyncWriteExt};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index))
        .route("/upload", post(upload));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn index() -> &'static str {
    "🐙 Welcome to OktoSync!"
}

async fn upload(mut multipart: Multipart) {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.file_name().unwrap().to_string();
        let data = field.bytes().await.unwrap();

        println!("Length of `{name}` is {} bytes.", data.len());
        if let Err(e) = save_file(&name, data).await {
            eprintln!("Failed to save file: {e}");
        };
    }
}

async fn save_file(file_name: &str, data: Bytes) -> std::io::Result<()> {
    let mut file = File::create(file_name).await?;
    file.write_all(&data).await?;
    Ok(())
}
