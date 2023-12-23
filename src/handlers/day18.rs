use anyhow::Context;
use axum::extract::{self, State};
use axum::Json;
use serde::Serialize;
use sqlx::prelude::FromRow;
use sqlx::{Postgres, QueryBuilder};

use crate::app_state::AppState;
use crate::prelude::*;
use crate::repo::Region;

pub async fn reset_orders_and_regions_db(State(state): State<AppState>) -> Result<()> {
    let mut transaction = state.db.begin().await.context("start transaction")?;

    sqlx::query("DROP TABLE IF EXISTS regions")
        .execute(&mut *transaction)
        .await
        .context("drop regions table")?;

    sqlx::query("DROP TABLE IF EXISTS orders")
        .execute(&mut *transaction)
        .await
        .context("drop orders table")?;

    sqlx::query(
        "
CREATE TABLE regions (
  id INT PRIMARY KEY,
  name VARCHAR(50)
)",
    )
    .execute(&mut *transaction)
    .await
    .context("create regions table")?;

    sqlx::query(
        "
CREATE TABLE orders (
  id INT PRIMARY KEY,
  region_id INT,
  gift_name VARCHAR(50),
  quantity INT
)",
    )
    .execute(&mut *transaction)
    .await
    .context("create orders table")?;

    transaction.commit().await.context("commit transaction")?;

    Ok(())
}

pub async fn create_regions(
    State(state): State<AppState>,
    extract::Json(regions): extract::Json<Vec<Region>>,
) -> Result<()> {
    tracing::debug!("Creating regions: {:?}", regions);

    if regions.is_empty() {
        return Ok(());
    }

    let mut query_builder = QueryBuilder::<Postgres>::new("INSERT INTO regions (id, name)");
    query_builder.push_values(regions, |mut b, region| {
        b.push_bind(region.id);
        b.push_bind(region.name);
    });
    tracing::debug!("Query: {}", query_builder.sql());

    query_builder.build().execute(&state.db).await?;

    Ok(())
}

#[derive(Debug, Serialize, FromRow)]
pub struct RegionsOrdersSummary {
    region: String,
    total: i64,
}
pub async fn get_regions_orders_summary(
    State(state): State<AppState>,
) -> Result<Json<Vec<RegionsOrdersSummary>>> {
    let data = sqlx::query_as::<_, RegionsOrdersSummary>(
        "
    SELECT regions.name AS region, SUM(orders.quantity) AS total 
    FROM orders INNER JOIN regions ON orders.region_id = regions.id 
    GROUP BY 1 
    ORDER BY 1",
    )
    .fetch_all(&state.db)
    .await?;

    Ok(Json(data))
}
