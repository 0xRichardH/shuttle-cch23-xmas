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
use shuttle_secrets::SecretStore;
use sqlx::PgPool;
use tower_http::{services::ServeDir, trace::TraceLayer};

#[shuttle_runtime::main]
async fn main(
    #[shuttle_persist::Persist] persist: PersistInstance,
    #[shuttle_shared_db::Postgres] db_pool: PgPool,
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
) -> shuttle_axum::ShuttleAxum {
    let app_state = AppState::new(secret_store, persist, db_pool);

    let router = Router::new()
        .route("/", get(handlers::hello_world))
        .route("/-1/error", get(handlers::fake_error))
        .route("/1/*rest", get(handlers::recalibrate_packet_id))
        .route("/4/strength", post(handlers::reindeer_strength))
        .route("/4/contest", post(handlers::reindeer_contest))
        .route("/5", post(handlers::slice_the_loop))
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
        .route("/13/sql", get(handlers::db_health_check))
        .route("/13/reset", post(handlers::reset_orders_db))
        .route("/13/orders", post(handlers::create_orders))
        .route("/13/orders/total", get(handlers::get_total_orders))
        .route("/13/orders/popular", get(handlers::get_popular_gift))
        .route("/14/unsafe", post(handlers::render_unsafe_html))
        .route("/14/safe", post(handlers::render_safe_html))
        .route("/15/nice", post(handlers::password_validator))
        .route("/15/game", post(handlers::password_game_validator))
        .route("/18/reset", post(handlers::reset_orders_and_regions_db))
        .route("/18/orders", post(handlers::create_orders))
        .route("/18/regions", post(handlers::create_regions))
        .route(
            "/18/regions/total",
            get(handlers::get_regions_orders_summary),
        )
        .route(
            "/18/regions/top_list/:number",
            get(handlers::get_regions_top_gifts),
        )
        .route("/19/ws/ping", get(handlers::ws_handler))
        .route("/19/ws/room/:room_id/user/:user", get(handlers::chatroom))
        .route("/19/views", get(handlers::get_tweet_view_count))
        .route("/19/reset", post(handlers::reset_tweet_view_count))
        .route("/20/archive_files", post(handlers::count_archive_files))
        .route(
            "/20/archive_files_size",
            post(handlers::count_archive_files_size),
        )
        .route("/20/cookie", post(handlers::get_cookie_from_archive_file))
        .route("/21/coords/:cell_id", get(handlers::parse_coords))
        .route(
            "/21/country/:cell_id",
            get(handlers::get_country_from_coords),
        )
        .route("/22/integers", post(handlers::get_gift_emojis))
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
