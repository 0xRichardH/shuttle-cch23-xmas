use std::collections::HashMap;

use anyhow::Context;
use axum::{debug_handler, extract, http::StatusCode, response::IntoResponse, Json};
use axum_extra::extract::CookieJar;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::trace;

use crate::{errors::AppError, CookieIngredient, Reindeer, ReindeerContestStats};

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
    let recipe = get_cookies_recipe(jar)?;

    Ok(recipe)
}

#[derive(Debug, Deserialize)]
struct BakeCookieRequest {
    recipe: HashMap<String, u32>,
    pantry: HashMap<String, u32>,
}

pub async fn bake_cookies(jar: CookieJar) -> Result<Json<serde_json::Value>, AppError> {
    let recipe_and_pantry = get_cookies_recipe(jar)?;
    let recipe_and_pantry = serde_json::from_str::<BakeCookieRequest>(&recipe_and_pantry)?;
    let recipe = CookieIngredient::from(&recipe_and_pantry.recipe);
    let pantry = CookieIngredient::from(&recipe_and_pantry.pantry);
    if recipe.is_none() || pantry.is_none() {
        return Ok(Json(json!({
            "cookies": 0,
            "pantry": json!(recipe_and_pantry.pantry),
        })));
    }
    let recipe = recipe.unwrap();
    let pantry = pantry.unwrap();

    // calculate how many cookies we can bake
    let cookies_count = vec![
        pantry.flour / recipe.flour,
        pantry.sugar / recipe.sugar,
        pantry.butter / recipe.butter,
        pantry.baking_powder / recipe.baking_powder,
        pantry.chocolate_chips / recipe.chocolate_chips,
    ]
    .into_iter()
    .min();

    let mut remain_pantry = CookieIngredient::default();
    if let Some(cookies_count) = cookies_count {
        remain_pantry.flour = pantry.flour - recipe.flour * cookies_count;
        remain_pantry.sugar = pantry.sugar - recipe.sugar * cookies_count;
        remain_pantry.butter = pantry.butter - recipe.butter * cookies_count;
        remain_pantry.baking_powder = pantry.baking_powder - recipe.baking_powder * cookies_count;
        remain_pantry.chocolate_chips =
            pantry.chocolate_chips - recipe.chocolate_chips * cookies_count;
    }

    Ok(Json(json!({
        "cookies": cookies_count,
        "pantry": {
            "flour": remain_pantry.flour,
            "sugar": remain_pantry.sugar,
            "butter": remain_pantry.butter,
            "baking powder": remain_pantry.baking_powder,
            "chocolate chips": remain_pantry.chocolate_chips,
        }
    })))
}

fn get_cookies_recipe(jar: CookieJar) -> anyhow::Result<String> {
    let mut recipe = String::new();
    if let Some(recipe_result) = jar.get("recipe").map(|c| base64::decode(c.value())) {
        recipe = String::from_utf8(recipe_result?).context("convert to string recipe")?;
    }

    Ok(recipe)
}
