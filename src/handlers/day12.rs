use std::time::SystemTime;

use anyhow::Context;
use axum::extract::{Path, State};

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
