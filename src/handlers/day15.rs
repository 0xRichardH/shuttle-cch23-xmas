use crate::prelude::*;
use axum::{extract, Json};
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize, Debug)]
pub struct PasswordInupt {
    #[serde(rename = "input")]
    pub password: String,
}

pub async fn password_validator(
    extract::Json(payload): extract::Json<PasswordInupt>,
) -> Result<Json<serde_json::Value>> {
    tracing::debug!("password_validator: {:?}", payload);

    if is_valid_password(payload.password.as_str()) {
        Ok(Json(json!({"result": "nice"})))
    } else {
        let msg = json!({"result": "naughty"});
        Err(AppError::BadRequest(msg.to_string()))
    }
}

fn is_valid_password(password: &str) -> bool {
    let vowels = "aeiouy";
    let mut vowel_count = 0;
    let mut has_consecutive = false;
    let forbidden_substrings = ["ab", "cd", "pq", "xy"];

    // Check for forbidden substrings
    for substring in &forbidden_substrings {
        if password.contains(substring) {
            return false;
        }
    }

    // Check for vowels and consecutive characters
    let mut chars = password.chars().peekable();
    while let Some(ch) = chars.next() {
        if vowels.contains(ch) {
            vowel_count += 1;
        }
        if let Some(&next_ch) = chars.peek() {
            if ch == next_ch && ch.is_alphabetic() {
                has_consecutive = true;
            }
        }
    }

    vowel_count >= 3 && has_consecutive
}
