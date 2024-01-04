use std::collections::HashMap;

use anyhow::Context;
use axum::extract::{self, Path, State};
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
        r#"
        
        CREATE TABLE regions ( id INT PRIMARY KEY, name varchar(50))"#,
    )
    .execute(&mut *transaction)
    .await
    .context("create regions table")?;

    sqlx::query(
        r#"
        CREATE TABLE orders ( id INT PRIMARY KEY, region_id INT, gift_name varchar(50), quantity INT
)"#,
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
        r#"
    SELECT regions.name AS region, SUM(orders.quantity) AS total 
    FROM orders INNER JOIN regions ON orders.region_id = regions.id 
    GROUP BY 1 
    ORDER BY 1"#,
    )
    .fetch_all(&state.db)
    .await?;

    Ok(Json(data))
}

#[derive(Debug, Serialize)]
pub struct RegionsTopGifts {
    region: String,
    top_gifts: Vec<String>,
}

pub async fn get_regions_top_gifts(
    State(state): State<AppState>,
    Path(number): Path<u32>,
) -> Result<Json<Vec<RegionsTopGifts>>> {
    let mut sql = "SELECT name, NULL FROM regions ORDER BY name";
    if number > 0 {
        sql = r#"
WITH added_row_number AS (
  SELECT 
    regions.name AS region, 
    orders.gift_name AS gift_name, 
    ROW_NUMBER() OVER(PARTITION BY regions.name ORDER BY SUM(orders.quantity) DESC, orders.gift_name ASC) AS row_number
  FROM orders FULL JOIN regions ON orders.region_id = regions.id
  GROUP BY 1, 2
  ORDER BY 1, 3 DESC
)
SELECT region, gift_name FROM added_row_number WHERE row_number <= $1 ORDER BY region, row_number;
        "#;
    }

    let data = sqlx::query_as::<_, (Option<String>, Option<String>)>(sql)
        .bind(number as i32)
        .fetch_all(&state.db)
        .await?;
    tracing::debug!("Top gifts of Regions (raw data): {:?}", data);

    let mut top_gifts = HashMap::<String, Vec<String>>::new();
    for (region, gift) in data {
        let Some(region) = region else {
            continue;
        };
        let v = top_gifts.entry(region).or_default();
        if let Some(gift) = gift {
            v.push(gift);
        }
    }
    let mut result = top_gifts
        .into_iter()
        .map(|(r, g)| RegionsTopGifts {
            region: r,
            top_gifts: g,
        })
        .collect::<Vec<RegionsTopGifts>>();
    result.sort_by(|a, b| a.region.cmp(&b.region));
    tracing::debug!(
        "Top gifts of Regions: {:?}",
        serde_json::to_string(&result)?
    );

    Ok(Json(result))
}
