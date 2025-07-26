use std::str::FromStr;

use serde::Deserialize;

use base64::{Engine as _, engine::general_purpose};
use validator::{Validate, ValidationError};

#[derive(Validate)]
pub struct User {
    pub id: Option<i64>,
    #[validate(length(min = 6))]
    pub username: String,
    #[validate(email)]
    pub email: String,
    #[validate(custom(function = "validate_public_key"))]
    pub public_key: String,
}

#[derive(Debug, Deserialize)]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub public_key: String,
}

fn validate_public_key(key: &str) -> Result<(), ValidationError> {
    if is_valid_openssh_key(key) {
        return Ok(());
    }

    if is_valid_pem_key(key) {
        return Ok(());
    }

    if is_valid_base64_raw_key(key) {
        return Ok(());
    }

    Err(ValidationError::new("invalid_public_key"))
}

fn is_valid_openssh_key(key: &str) -> bool {
    ssh_key::PublicKey::from_str(key).is_ok()
}

fn is_valid_pem_key(key: &str) -> bool {
    pem::parse(key)
        .map(|parsed| parsed.tag().contains("PUBLIC KEY"))
        .unwrap_or(false)
}

fn is_valid_base64_raw_key(key: &str) -> bool {
    match general_purpose::STANDARD.decode(key) {
        Ok(bytes) => matches!(bytes.len(), 32 | 33 | 64 | 65),
        Err(_) => false,
    }
}
