use axum::{
    extract::{MatchedPath, Request},
    routing::{get, post},
    Router,
};
use cch23_xmas::handlers;
use tower_http::trace::TraceLayer;

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(handlers::hello_world))
        .route("/-1/error", get(handlers::fake_error))
        .route("/1/*rest", get(handlers::recalibrate_packet_id))
        .route("/4/strength", post(handlers::reindeer_strength))
        .route("/4/contest", post(handlers::reindeer_contest))
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
