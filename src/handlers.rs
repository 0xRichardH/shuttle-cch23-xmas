use axum::{debug_handler, extract, http::StatusCode, response::IntoResponse};

use crate::Reindeer;

pub async fn hello_world() -> &'static str {
    "Hello, world!"
}

pub async fn fake_error() -> impl IntoResponse {
    (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error")
}

pub async fn recalibrate_packet_id(
    extract::Path(rest): extract::Path<String>,
) -> impl IntoResponse {
    let numbers = rest
        .split('/')
        .flat_map(|s| s.parse().ok())
        .collect::<Vec<u32>>();

    if numbers.len() > 20 {
        return (StatusCode::NOT_FOUND, "Not Found".to_string());
    }

    let mut xor_rsult = 0;
    for n in numbers {
        xor_rsult ^= n;
    }
    (StatusCode::OK, xor_rsult.pow(3).to_string())
}

pub async fn reindeer_strength(
    extract::Json(reindeers): extract::Json<Vec<Reindeer>>,
) -> impl IntoResponse {
    let strength = reindeers.iter().map(|r| r.strength()).sum::<u32>();
    (StatusCode::OK, strength.to_string())
}

#[debug_handler]
pub async fn reindeer_contest(
    extract::Json(reindeers): extract::Json<Vec<Reindeer>>,
) -> impl IntoResponse {
    (StatusCode::OK, "")
}
