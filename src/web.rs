pub mod handlers {
    use axum::{body::Bytes, extract::Multipart};
    use tokio::{fs::File, io::AsyncWriteExt};

    pub async fn index() -> &'static str {
        "🐙 Welcome to OktoSync!"
    }

    pub async fn upload(mut multipart: Multipart) {
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
}
