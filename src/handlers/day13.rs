use axum::extract::State;

use crate::{app_state::AppState, prelude::*};

pub async fn db_health_check(State(state): State<AppState>) -> Result<String> {
    let (r,) = sqlx::query_as::<_, (i32,)>("SELECT 20231213")
        .fetch_one(&state.db)
        .await?;
    Ok(r.to_string())
}
