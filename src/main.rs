use axum::{
    routing::{get, post},
    Router,
};
use cch23_xmas::handlers;

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(handlers::hello_world))
        .route("/-1/error", get(handlers::fake_error))
        .route("/1/*rest", get(handlers::recalibrate_packet_id))
        .route("/4/strength", post(handlers::reindeer_strength))
        .route("/4/contest", post(handlers::reindeer_contest));

    Ok(router.into())
}
