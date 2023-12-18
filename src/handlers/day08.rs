use crate::prelude::*;
use axum::extract;

use crate::Pokemon;

pub async fn get_pokemon_weight(extract::Path(number): extract::Path<u64>) -> Result<String> {
    let pokemon: Pokemon = get_pokemon_by_number(number).await?;

    Ok(convert_hg_to_kg(pokemon.weight).to_string())
}

pub async fn drop_pokemon(extract::Path(number): extract::Path<u64>) -> Result<String> {
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
