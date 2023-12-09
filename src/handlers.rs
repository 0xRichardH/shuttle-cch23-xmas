use anyhow::Context;
use axum::{debug_handler, extract, http::StatusCode, response::IntoResponse, Json};
use axum_extra::extract::CookieJar;
use serde::Serialize;
use tracing::trace;

use crate::{errors::AppError, Reindeer, ReindeerContestStats};

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
    #[serde(rename(serialize = "elf on a shelf"))]
    elf_on_shelf: usize,
    #[serde(rename(serialize = "shelf with no elf on it"))]
    shelf_with_no_elf: usize,
}
pub async fn count_elf(body: String) -> Json<CountElfResponse> {
    trace!("count_elf: {body}");

    let elf = body.match_indices("elf").count();
    let elf_on_shelf = body.matches("elf on a shelf").count();
    let shelf_with_no_elf = body.match_indices("shelf").count() - elf_on_shelf;

    Json(CountElfResponse {
        elf,
        elf_on_shelf,
        shelf_with_no_elf,
    })
}

pub async fn cookies_recipe(jar: CookieJar) -> Result<String, AppError> {
    let mut recipe = String::new();
    if let Some(recipe_result) = jar.get("recipe").map(|c| base64::decode(c.value())) {
        recipe = String::from_utf8(recipe_result?).context("convert to string recipe")?;
    }

    Ok(recipe)
}
