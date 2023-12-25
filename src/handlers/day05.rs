use crate::prelude::*;
use axum::{extract::Query, Json};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct SliceTheLoopQuery {
    offset: Option<usize>,
    limit: Option<usize>,
    split: Option<usize>,
}

pub async fn slice_the_loop(
    slice_query: Query<SliceTheLoopQuery>,
    Json(payload): Json<Vec<String>>,
) -> Result<Json<serde_json::Value>> {
    let mut offset = 0;
    let mut limit = payload.len();

    if let Some(offset_query) = slice_query.offset {
        offset = offset_query;
    }
    if let Some(limit_query) = slice_query.limit {
        limit = limit_query;
    }

    let split = slice_query.split.unwrap_or(0);
    if split > 0 {
        let result = payload[offset..]
            .chunks(split)
            .take(limit)
            .map(|el| el.to_vec())
            .collect::<Vec<_>>();

        Ok(Json(serde_json::to_value(result)?))
    } else {
        let result = payload.iter().skip(offset).take(limit).collect::<Vec<_>>();
        Ok(Json(serde_json::to_value(result)?))
    }
}
