use axum::{
    debug_handler, extract,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use cch23_xmas::Reindeer;

async fn hello_world() -> &'static str {
    "Hello, world!"
}

async fn fake_error() -> impl IntoResponse {
    (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error")
}

async fn recalibrate_packet_id(extract::Path(rest): extract::Path<String>) -> impl IntoResponse {
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

async fn reindeer_strength(
    extract::Json(reindeers): extract::Json<Vec<Reindeer>>,
) -> impl IntoResponse {
    let strength = reindeers.iter().map(|r| r.strength()).sum::<u32>();
    (StatusCode::OK, strength.to_string())
}

#[debug_handler]
async fn reindeer_contest(
    extract::Json(reindeers): extract::Json<Vec<Reindeer>>,
) -> impl IntoResponse {
    (StatusCode::OK, "")
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/-1/error", get(fake_error))
        .route("/1/*rest", get(recalibrate_packet_id))
        .route("/4/strength", post(reindeer_strength))
        .route("/4/contest", post(reindeer_contest));

    Ok(router.into())
}
