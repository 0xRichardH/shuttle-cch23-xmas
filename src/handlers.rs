use axum::{debug_handler, extract, http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;
use tracing::trace;

use crate::{Reindeer, ReindeerContestStats};

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
) -> Json<ReindeerContestStats> {
    let mut fastest_idx = 0;
    let mut tallest_idx = 0;
    let mut magician_idx = 0;
    let mut consumer_idx = 0;

    reindeers.iter().enumerate().for_each(|(idx, deer)| {
        if deer.speed() > reindeers[fastest_idx].speed() {
            fastest_idx = idx;
        }

        if deer.height() > reindeers[tallest_idx].height() {
            tallest_idx = idx;
        }

        if deer.snow_magic_power() > reindeers[magician_idx].snow_magic_power() {
            magician_idx = idx;
        }

        if deer.candy_eaten_yesterday() > reindeers[consumer_idx].candy_eaten_yesterday() {
            consumer_idx = idx;
        }
    });
    let stats = ReindeerContestStats::new(
        reindeers[fastest_idx].clone(),
        reindeers[tallest_idx].clone(),
        reindeers[magician_idx].clone(),
        reindeers[consumer_idx].clone(),
    );
    Json(stats)
}

#[derive(Serialize)]
pub struct CountElfResponse {
    elf: usize,
}
pub async fn count_elf(body: String) -> Json<CountElfResponse> {
    trace!("count_elf: {body}");

    let count = body.match_indices("elf").count();

    Json(CountElfResponse { elf: count })
}
