use std::collections::HashMap;

use anyhow::{bail, Context};
use axum::{
    debug_handler, extract,
    http::{StatusCode, Uri},
    response::IntoResponse,
    Json,
};
use axum_extra::{headers::Cookie, TypedHeader};
use base64::{engine::general_purpose, Engine};
use serde::{Deserialize, Serialize};
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

pub async fn cookies_recipe(TypedHeader(cookie): TypedHeader<Cookie>) -> Result<String, AppError> {
    let recipe = get_cookies_recipe(&cookie)?;

    Ok(recipe)
}

#[derive(Debug, Deserialize)]
struct BakeCookieRequest {
    recipe: HashMap<String, u64>,
    pantry: HashMap<String, u64>,
}

#[debug_handler]
pub async fn bake_cookies(
    TypedHeader(cookie): TypedHeader<Cookie>,
) -> Result<Json<serde_json::Value>, AppError> {
    let recipe_and_pantry = get_cookies_recipe(&cookie)?;
    // tracing::debug!("recipe_and_pantry {:?}", recipe_and_pantry);

    let recipe_and_pantry = serde_json::from_str::<BakeCookieRequest>(&recipe_and_pantry)?;
    let mut pantry = recipe_and_pantry.pantry;
    let recipe = recipe_and_pantry.recipe;

    // calculate how many cookies we can bake
    let cookies_count = recipe.iter().fold(u64::MAX, |count, (ingredient, amount)| {
        if let Some(avaliable) = pantry.get(ingredient) {
            if let Some(c) = avaliable.checked_div(*amount) {
                return count.min(c);
            }
        }

        0
    });

    pantry.iter_mut().for_each(|(key, value)| {
        *value -= cookies_count * recipe.get(key).unwrap_or(&0);
    });

    Ok(Json(json!({
        "cookies": cookies_count,
        "pantry": pantry
    })))
}

fn get_cookies_recipe(cookie: &Cookie) -> anyhow::Result<String> {
    let recipe_cookie = cookie.get("recipe");
    if recipe_cookie.is_none() {
        bail!("recipe cookie not found");
    }
    let recipe_result = general_purpose::STANDARD
        .decode(recipe_cookie.unwrap())
        .context("decode recipe")?;
    let recipe = String::from_utf8(recipe_result).context("convert to string recipe")?;

    Ok(recipe)
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
