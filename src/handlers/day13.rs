use anyhow::Context;
use axum::{
    extract::{self, State},
    Json,
};
use serde_json::json;
use sqlx::{Postgres, QueryBuilder};

use crate::{app_state::AppState, prelude::*, repo::Order};

pub async fn db_health_check(State(state): State<AppState>) -> Result<String> {
    let (r,) = sqlx::query_as::<_, (i32,)>("SELECT 20231213")
        .fetch_one(&state.db)
        .await?;
    Ok(r.to_string())
}

pub async fn reset_orders_bd(State(state): State<AppState>) -> Result<()> {
    let mut transaction = state.db.begin().await.context("start transaction")?;

    sqlx::query("DROP TABLE IF EXISTS orders")
        .execute(&mut *transaction)
        .await
        .context("drop table")?;

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
    .context("create table")?;

    transaction.commit().await.context("commit transaction")?;

    Ok(())
}

pub async fn create_orders(
    State(state): State<AppState>,
    extract::Json(orders): extract::Json<Vec<Order>>,
) -> Result<()> {
    tracing::debug!("Creating orders: {:?}", orders);

    let mut query_builder =
        QueryBuilder::<Postgres>::new("INSERT INTO orders (id, region_id, gift_name, quantity)");

    query_builder.push_values(orders, |mut b, order| {
        b.push_bind(order.id);
        b.push_bind(order.region_id);
        b.push_bind(order.gift_name);
        b.push_bind(order.quantity);
    });

    tracing::debug!("Query: {}", query_builder.sql());

    let query = query_builder.build();
    query.execute(&state.db).await?;

    Ok(())
}

pub async fn get_total_orders(State(state): State<AppState>) -> Result<Json<serde_json::Value>> {
    let (r,) = sqlx::query_as::<_, (i64,)>("SELECT SUM(quantity) FROM orders")
        .fetch_one(&state.db)
        .await?;
    Ok(Json(json!({ "total": r })))
}

pub async fn get_popular_gift(State(state): State<AppState>) -> Result<Json<serde_json::Value>> {
    let result = sqlx::query_as::<_, (String,)>("SELECT gift_name, SUM(quantity) AS quantity FROM orders GROUP BY gift_name ORDER BY quantity DESC LIMIT 1")
        .fetch_optional(&state.db)
        .await?;
    if let Some((name,)) = result {
        Ok(Json(json!({ "popular": name })))
    } else {
        Ok(Json(json!({ "popular": null })))
    }
}
