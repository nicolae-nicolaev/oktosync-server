pub mod handlers {
    use std::path::Path;

    use axum::{body::Bytes, extract::Multipart};
    use tokio::{fs::File, io::AsyncWriteExt};

    pub async fn index() -> &'static str {
        "🐙 Welcome to OktoSync!"
    }

    pub async fn upload(mut multipart: Multipart) {
        loop {
            match multipart.next_field().await {
                Ok(Some(field)) => {
                    let file_name = match field.file_name() {
                        Some(name) => name.to_string(),
                        None => {
                            eprintln!("⚠️ Skipping field with no filename.");
                            continue;
                        }
                    };

                    let Ok(data) = field.bytes().await else {
                        eprintln!("⚠️ Skipping field `{file_name}` due to read error.");
                        continue;
                    };

                    if let Err(e) = save_file(&file_name, data).await {
                        eprintln!("❗ Failed to save file: {e}")
                    }
                }
                Ok(None) => break,
                Err(e) => {
                    eprintln!("❗ Multipart error: {e}. Skipping to next.");
                    continue;
                }
            }
        }
    }

    async fn save_file(file_name: &str, data: Bytes) -> std::io::Result<()> {
        let upload_dir = Path::new("uploads");
        let sanitized_filename = file_name.replace("../", "").replace("/", "");
        let safe_path = upload_dir.join(sanitized_filename);

        let mut file = File::create(safe_path).await?;
        file.write_all(&data).await?;

        Ok(())
    }
}
