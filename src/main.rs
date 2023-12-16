use axum::{
    extract::{MatchedPath, Request},
    routing::{get, post},
    Router,
};
use cch23_xmas::{
    app_state::AppState,
    handlers::{self},
};
use shuttle_persist::PersistInstance;
use tower_http::{services::ServeDir, trace::TraceLayer};

#[shuttle_runtime::main]
async fn main(#[shuttle_persist::Persist] persist: PersistInstance) -> shuttle_axum::ShuttleAxum {
    let app_state = AppState::new(persist);

    let router = Router::new()
        .route("/", get(handlers::hello_world))
        .route("/-1/error", get(handlers::fake_error))
        .route("/1/*rest", get(handlers::recalibrate_packet_id))
        .route("/4/strength", post(handlers::reindeer_strength))
        .route("/4/contest", post(handlers::reindeer_contest))
        .route("/6", post(handlers::count_elf))
        .route("/7/decode", get(handlers::cookies_recipe))
        .route("/7/bake", get(handlers::bake_cookies))
        .route("/8/weight/:number", get(handlers::get_pokemon_weight))
        .route("/8/drop/:number", get(handlers::drop_pokemon))
        .nest_service("/11/assets/", ServeDir::new("assets"))
        .route("/11/red_pixels", post(handlers::red_pixels))
        .route("/12/save/:time_key", post(handlers::persist_time))
        .route("/12/load/:time_key", get(handlers::load_time))
        .route("/12/ulids", post(handlers::convert_ulids_to_uuids))
        .route("/12/ulids/:weekday", post(handlers::count_ulids))
        .fallback(handlers::not_found_handler)
        .with_state(app_state)
        .layer(
            TraceLayer::new_for_http().make_span_with(|request: &Request<_>| {
                // Log the matched route's path (with placeholders not filled in).
                // Use request.uri() or OriginalUri if you want the real path.
                let matched_path = request
                    .extensions()
                    .get::<MatchedPath>()
                    .map(MatchedPath::as_str);

                tracing::info_span!(
                    "http_request",
                    method = ?request.method(),
                    matched_path,
                    some_other_field = tracing::field::Empty,
                )
            }),
        );

    Ok(router.into())
}
