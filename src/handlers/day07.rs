use std::collections::HashMap;

use anyhow::{bail, Context};
use axum::{debug_handler, Json};
use axum_extra::{headers::Cookie, TypedHeader};
use base64::{engine::general_purpose, Engine};
use serde::Deserialize;
use serde_json::json;

use crate::prelude::*;

#[derive(Debug, Deserialize)]
struct BakeCookieRequest {
    recipe: HashMap<String, u64>,
    pantry: HashMap<String, u64>,
}

/// task 1
pub async fn cookies_recipe(TypedHeader(cookie): TypedHeader<Cookie>) -> Result<String> {
    let recipe = get_cookies_recipe(&cookie)?;

    Ok(recipe)
}

/// task 2 and 3
#[debug_handler]
pub async fn bake_cookies(
    TypedHeader(cookie): TypedHeader<Cookie>,
) -> Result<Json<serde_json::Value>> {
    let recipe_and_pantry =
        get_cookies_recipe(&cookie).context("get recipe_and_pantry from cookies")?;
    tracing::debug!("recipe_and_pantry {:?}", recipe_and_pantry);

    let recipe_and_pantry = serde_json::from_str::<BakeCookieRequest>(&recipe_and_pantry)?;
    let mut pantry = recipe_and_pantry.pantry;
    let recipe = recipe_and_pantry.recipe;

    // calculate how many cookies we can bake
    let cookies_count = recipe.iter().fold(u64::MAX, |count, (ingredient, needed)| {
        if let Some(avaliable) = pantry.get(ingredient) {
            if let Some(c) = avaliable.checked_div(*needed) {
                return count.min(c);
            }
        }

        count.min(u64::MAX)
    });

    pantry.iter_mut().for_each(|(key, value)| {
        *value -= cookies_count * recipe.get(key).unwrap_or(&0);
    });

    let res = Json(json!({
        "cookies": cookies_count,
        "pantry": pantry
    }));

    Ok(res)
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
