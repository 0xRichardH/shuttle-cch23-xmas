use std::time::SystemTime;

use anyhow::Context;
use axum::{
    extract::{self, Path, State},
    Json,
};
use chrono::{DateTime, Datelike, Utc, Weekday};
use serde_json::json;
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
) -> Result<Json<serde_json::Value>> {
    if !(0..=6).contains(&weekday) {
        return Err(AppError::BadRequest(format!(
            "Weekday must be between 0 and 6, got {}",
            weekday
        )));
    }

    let ulids = ulids
        .into_iter()
        .flat_map(|ulid| Ulid::from_string(ulid.as_str()).ok());

    let now: DateTime<Utc> = Utc::now();
    let (mut in_christmas_eve, mut is_weekday, mut in_the_future, mut lsb_is_1) = (0, 0, 0, 0);
    for ulid in ulids {
        let datetime: DateTime<Utc> = ulid.datetime().into();
        if datetime.month() == 12 && datetime.day() == 24 {
            in_christmas_eve += 1;
        }
        if datetime.weekday().num_days_from_monday() == weekday as u32 {
            is_weekday += 1;
        }
        if datetime > now {
            in_the_future += 1;
        }
        let lsb = ulid.to_bytes()[15] & 1;
        if lsb == 1 {
            lsb_is_1 += 1;
        }
    }

    let result = json!({
        "christmas eve": in_christmas_eve,
        "weekday": is_weekday,
        "in the future": in_the_future,
        "LSB is 1": lsb_is_1,
    });
    Ok(Json(result))
}
