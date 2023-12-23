use crate::prelude::*;
use axum::{extract, http::StatusCode, Json};
use regex::Regex;
use serde::Deserialize;
use serde_json::json;
use sha2::{Digest, Sha256};

#[derive(Deserialize, Debug)]
pub struct RequestInput {
    pub input: String,
}

pub async fn password_validator(
    extract::Json(payload): extract::Json<RequestInput>,
) -> Result<Json<serde_json::Value>> {
    tracing::debug!("password_validator: {:?}", payload);

    if is_valid_password(payload.input.as_str()) {
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

pub async fn password_game_validator(
    extract::Json(payload): extract::Json<RequestInput>,
) -> Result<Json<serde_json::Value>> {
    tracing::debug!("password_game_validator: {:?}", payload);

    valid_game_input(payload.input.as_str())?;

    Ok(Json(
        json!({"result": "nice", "reason": "that's a nice password"}),
    ))
}

fn valid_game_input(input: &str) -> Result<()> {
    // Rule 1: must be at least 8 characters long
    if input.len() < 8 {
        return Err(AppError::InvalidPasswordGameInput(
            StatusCode::BAD_REQUEST,
            "8 chars".to_string(),
        ));
    }

    let mut has_contain_uppercase = false;
    let mut has_contain_lowercase = false;
    let mut has_contain_digit = false;
    let mut digits_counter = 0;
    let mut joy = Vec::new();
    let mut has_contain_unicode = false;
    let mut has_emoji = false;
    for c in input.chars() {
        if c.is_uppercase() {
            has_contain_uppercase = true;
        }
        if c.is_lowercase() {
            has_contain_lowercase = true;
        }
        if c.is_ascii_digit() {
            has_contain_digit = true;
            digits_counter += 1;
        }

        if c == 'j' || c == 'o' || c == 'y' {
            joy.push(c);
        }

        if (0x2980..=0x2BFF).contains(&(c as u32)) {
            has_contain_unicode = true;
        }

        if emojis::get(c.to_string().as_str()).is_some() {
            has_emoji = true;
        }
    }

    // Rule 2: must contain uppercase letters, lowercase letters, and digits
    if !has_contain_uppercase || !has_contain_lowercase || !has_contain_digit {
        return Err(AppError::InvalidPasswordGameInput(
            StatusCode::BAD_REQUEST,
            "more types of chars".to_string(),
        ));
    }

    // Rule 3: must contain at least 5 digits
    if digits_counter < 5 {
        return Err(AppError::InvalidPasswordGameInput(
            StatusCode::BAD_REQUEST,
            "55555".to_string(),
        ));
    }

    // Rule 4: all integers (sequences of consecutive digits) in the string must add up to 2023
    if !sum_of_digits_equals_2023(input) {
        return Err(AppError::InvalidPasswordGameInput(
            StatusCode::BAD_REQUEST,
            "math is hard".to_string(),
        ));
    }

    // Rule 5: must contain the letters j, o, and y in that order and in no other order
    if joy.len() < 3 || joy.iter().collect::<String>() != "joy" {
        return Err(AppError::InvalidPasswordGameInput(
            StatusCode::NOT_ACCEPTABLE,
            "not joyful enough".to_string(),
        ));
    }

    // Rule 6: must contain a letter that repeats with exactly one other letter between them (like xyx)
    if !has_repeating_pattern(input) {
        return Err(AppError::InvalidPasswordGameInput(
            StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS,
            "illegal: no sandwich".to_string(),
        ));
    }

    // Rule 7: must contain at least one unicode character in the range [U+2980, U+2BFF]
    if !has_contain_unicode {
        return Err(AppError::InvalidPasswordGameInput(
            StatusCode::RANGE_NOT_SATISFIABLE,
            "outranged".to_string(),
        ));
    }

    // Rule 8: must contain at least one emoji
    if !has_emoji {
        return Err(AppError::InvalidPasswordGameInput(
            StatusCode::UPGRADE_REQUIRED,
            "😳".to_string(),
        ));
    }

    // Rule 9: the hexadecimal representation of the sha256 hash of the string must end with an a
    let mut hasher = Sha256::new();
    hasher.update(input);
    let result = hasher.finalize();
    if !format!("{:x}", result).ends_with('a') {
        return Err(AppError::InvalidPasswordGameInput(
            StatusCode::IM_A_TEAPOT,
            "not a coffee brewer".to_string(),
        ));
    }

    Ok(())
}

fn sum_of_digits_equals_2023(input: &str) -> bool {
    let re = Regex::new(r"\d+").unwrap();
    re.find_iter(input)
        .map(|mat| mat.as_str().parse::<i32>().unwrap_or(0))
        .sum::<i32>()
        == 2023
}

fn has_repeating_pattern(input: &str) -> bool {
    let chars: Vec<char> = input.chars().collect();
    let len = chars.len();

    for i in 0..len {
        if !chars[i].is_alphabetic() {
            continue;
        }

        if i + 2 < len && chars[i] == chars[i + 2] {
            return true;
        }
    }

    false
}
