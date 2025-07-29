pub mod handlers {
    use std::path::{Component, Path, PathBuf};

    use axum::{body::Bytes, extract::Multipart};
    use tokio::{fs::File, io::AsyncWriteExt};

    pub async fn index() -> &'static str {
        "🐙 Welcome to OktoSync!"
    }

    pub async fn upload(mut multipart: Multipart) {
        let mut file_data: Option<Bytes> = None;
        let mut relative_path: Option<String> = None;

        while let Ok(Some(field)) = multipart.next_field().await {
            match field.name() {
                Some("file") => match field.bytes().await {
                    Ok(data) => file_data = Some(data),
                    Err(e) => {
                        eprintln!("⚠️ Failed to read file bytes: {e}");
                        return;
                    }
                },
                Some("path") => match field.text().await {
                    Ok(path) => relative_path = Some(path),
                    Err(e) => {
                        eprintln!("⚠️ Failed to read path field: {e}");
                        return;
                    }
                },
                _ => {
                    eprintln!("⚠️ Unknown field in multipart upload");
                    continue;
                }
            }
        }

        let Some(data) = file_data else {
            eprintln!("❗ Missing file data");
            return;
        };

        let Some(path) = relative_path else {
            eprintln!("❗ Missing relative path");
            return;
        };

        if let Err(e) = save_file(&path, data).await {
            eprintln!("❗ Failed to save file: {e}")
        }
    }

    async fn save_file(file_path: &str, data: Bytes) -> std::io::Result<()> {
        let upload_dir = match std::env::var("OKTOSYNC_UPLOAD_DIR") {
            Ok(dir) => {
                let path = Path::new(&dir);
                if path.is_absolute()
                    && !path.components().any(|c| matches!(c, Component::ParentDir))
                {
                    path.to_path_buf()
                } else {
                    eprintln!(
                        "⚠️ Invalid upload directory in OKTOSYNC_UPLOAD_DIR. Falling back to default."
                    );
                    PathBuf::from("uploads")
                }
            }
            Err(_) => PathBuf::from("uploads"),
        };

        // Normalize relative path
        let mut clean_path = PathBuf::new();
        for component in Path::new(file_path).components() {
            match component {
                Component::Normal(part) => clean_path.push(part),
                Component::CurDir => {}
                Component::ParentDir => {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::PermissionDenied,
                        "🚫 Parent path (..) is not allowed",
                    ));
                }
                Component::RootDir | Component::Prefix(_) => {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::PermissionDenied,
                        "🚫 Absolute paths are not allowed",
                    ));
                }
            }
        }

        let final_path = upload_dir.join(&clean_path);

        let final_path_canonicalized = match final_path.canonicalize() {
            Ok(path) => path,
            Err(_) => {
                return Err(std::io::Error::other(
                    "🚫 Error processing upload directory",
                ));
            }
        };

        let upload_dir_canonicalized = match upload_dir.canonicalize() {
            Ok(path) => path,
            Err(_) => {
                return Err(std::io::Error::other(
                    "🚫 Error processing upload directory",
                ));
            }
        };

        if !final_path_canonicalized.starts_with(upload_dir_canonicalized) {
            return Err(std::io::Error::new(
                std::io::ErrorKind::PermissionDenied,
                "🚫 Path escapes upload directory",
            ));
        }

        if let Some(parent) = final_path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }

        let mut file = File::create(final_path).await?;
        file.write_all(&data).await?;

        Ok(())
    }
}
