use std::time::SystemTime;

use anyhow::Context;
use axum::{
    extract::{self, Path, State},
    Json,
};
use ulid::Ulid;
use uuid::Uuid;

use crate::app_state::AppState;
use crate::prelude::*;

pub async fn persist_time(
    State(state): State<AppState>,
    Path(time_key): Path<String>,
) -> Result<()> {
    state
        .persist
        .save(time_key.as_str(), SystemTime::now())
        .context("Failed to persist time time")?;

    Ok(())
}

pub async fn load_time(
    State(state): State<AppState>,
    Path(time_key): Path<String>,
) -> Result<String> {
    let time = state
        .persist
        .load::<SystemTime>(time_key.as_str())
        .context("Failed to load persisted time")?;
    let elapsed = time.elapsed().context("elapsed SystemTime")?;

    Ok(elapsed.as_secs().to_string())
}

pub async fn convert_ulids_to_uuids(
    extract::Json(ulids): extract::Json<Vec<String>>,
) -> Result<Json<Vec<String>>> {
    let uuids = ulids
        .into_iter()
        .filter_map(|ulid| Ulid::from_string(ulid.as_str()).ok())
        .map(Uuid::from)
        .map(|uuid| uuid.to_string())
        .rev()
        .collect::<Vec<String>>();

    Ok(Json(uuids))
}

pub async fn count_ulids(
    Path(weekday): Path<u8>,
    extract::Json(ulids): extract::Json<Vec<String>>,
) -> Result<String> {
    if !(0..=6).contains(&weekday) {
        return Err(AppError::BadRequest(format!(
            "Weekday must be between 0 and 6, got {}",
            weekday
        )));
    }

    let ulids = ulids
        .into_iter()
        .flat_map(|ulid| Ulid::from_string(ulid.as_str()).ok())
        .collect::<Vec<Ulid>>();

    Ok(ulids.len().to_string())
}
