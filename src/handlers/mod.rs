mod day01;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;

pub use day01::*;
pub use day04::*;
pub use day05::*;
pub use day06::*;
pub use day07::*;
pub use day08::*;
pub use day11::*;
pub use day12::*;
pub use day13::*;
pub use day14::*;
pub use day15::*;
pub use day18::*;
pub use day19::*;
pub use day20::*;
pub use day21::*;
pub use day22::*;

use axum::{
    http::{StatusCode, Uri},
    Json,
};

use serde_json::json;

pub async fn not_found_handler(uri: Uri) -> (StatusCode, Json<serde_json::Value>) {
    tracing::info!("path not found: {}", uri.path());

    (
        StatusCode::NOT_FOUND,
        Json(json!( {
            "error": String::from("not_found"),
            "message": Some(format!("Requested path `{}` not found.", uri.path())),
        })),
    )
}

pub async fn hello_world() -> &'static str {
    "Hello, world!"
}
