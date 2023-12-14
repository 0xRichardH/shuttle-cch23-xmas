mod day07;
pub use day07::*;

use axum::{
    debug_handler, extract,
    http::{StatusCode, Uri},
    response::IntoResponse,
    Json,
};

use serde::Serialize;
use serde_json::json;
use tracing::trace;

use crate::{errors::AppError, Pokemon, Reindeer, ReindeerContestStats};

pub async fn not_found_handler(uri: Uri) -> (StatusCode, Json<serde_json::Value>) {
    tracing::info!("path not found: {}", uri.path());

    (
        StatusCode::NOT_FOUND,
        Json(json!( {
            "error": String::from("not_found"),
            "message": Some(format!("Requested path `{}` not found.", uri.path())),
        })),
    )
}

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
        .collect::<Vec<i32>>();

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
    tracing::debug!("reindeers: {:?}", reindeers);

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

    tracing::debug!("reindeers: {:?}", reindeers);

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

#[debug_handler]
pub async fn count_elf(body: String) -> Json<CountElfResponse> {
    trace!("count_elf request body: {body}");

    let elf = body.matches("elf").count();

    let elf_on_shelf_search = b"elf on a shelf";
    let elf_on_shelf = body
        .as_bytes()
        .windows(elf_on_shelf_search.len())
        .filter(|el| el == elf_on_shelf_search)
        .count();
    let shelf_with_no_elf = body.matches("shelf").count() - elf_on_shelf;

    Json(CountElfResponse {
        elf,
        elf_on_shelf,
        shelf_with_no_elf,
    })
}

pub async fn get_pokemon_weight(
    extract::Path(number): extract::Path<u64>,
) -> Result<String, AppError> {
    let pokemon: Pokemon = get_pokemon_by_number(number).await?;

    Ok(convert_hg_to_kg(pokemon.weight).to_string())
}

pub async fn drop_pokemon(extract::Path(number): extract::Path<u64>) -> Result<String, AppError> {
    let pokemon = get_pokemon_by_number(number).await?;
    let weight = convert_hg_to_kg(pokemon.weight);
    let result = weight * (9.825 * 20f64).sqrt();
    Ok(result.to_string())
}

async fn get_pokemon_by_number(number: u64) -> anyhow::Result<Pokemon> {
    let body = reqwest::get(format!("https://pokeapi.co/api/v2/pokemon/{number}"))
        .await?
        .bytes()
        .await?;
    let pokemon: Pokemon = serde_json::from_slice(body.as_ref())?;
    tracing::info!("get pokemon: {pokemon:?}");

    Ok(pokemon)
}

fn convert_hg_to_kg(hg: u32) -> f64 {
    // convert hectogram to kilogram
    hg as f64 / 10f64
}
