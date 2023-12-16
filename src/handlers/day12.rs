use std::time::SystemTime;

use anyhow::Context;
use axum::{
    extract::{self, Path, State},
    Json,
};
use ulid::Ulid;
use uuid::Uuid;

use crate::{app_state::AppState, errors::AppError};

pub async fn persist_time(
    State(state): State<AppState>,
    Path(time_key): Path<String>,
) -> anyhow::Result<(), AppError> {
    state
        .persist
        .save(time_key.as_str(), SystemTime::now())
        .context("Failed to persist time time")?;

    Ok(())
}

pub async fn load_time(
    State(state): State<AppState>,
    Path(time_key): Path<String>,
) -> anyhow::Result<String, AppError> {
    let time = state
        .persist
        .load::<SystemTime>(time_key.as_str())
        .context("Failed to load persisted time")?;
    let elapsed = time.elapsed().context("elapsed SystemTime")?;

    Ok(elapsed.as_secs().to_string())
}

pub async fn convert_ulids_to_uuids(
    extract::Json(ulids): extract::Json<Vec<String>>,
) -> anyhow::Result<Json<Vec<String>>, AppError> {
    let uuids = ulids
        .into_iter()
        .filter_map(|ulid| {
            Ulid::from_string(ulid.as_str())
                .context("parser ulid from string")
                .ok()
        })
        .map(Uuid::from)
        .map(|uuid| uuid.to_string())
        .rev()
        .collect::<Vec<String>>();

    Ok(Json(uuids))
}
