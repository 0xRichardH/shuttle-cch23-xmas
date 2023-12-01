use axum::{extract::Path, http::StatusCode, response::IntoResponse, routing::get, Router};

async fn hello_world() -> &'static str {
    "Hello, world!"
}

async fn fake_error() -> impl IntoResponse {
    (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error")
}

async fn recalibrate_packet_id(Path((num1, num2)): Path<(u32, u32)>) -> String {
    let xor_rsult = num1 ^ num2;
    xor_rsult.pow(3).to_string()
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/-1/error", get(fake_error))
        .route("/1/:num1/:num2", get(recalibrate_packet_id));

    Ok(router.into())
}
