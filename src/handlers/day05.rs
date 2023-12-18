use axum::{debug_handler, Json};
use serde::Serialize;
use tracing::trace;

#[derive(Serialize)]
pub struct CountElfResponse {
    elf: usize,
    #[serde(rename(serialize = "elf on a shelf"))]
    elf_on_shelf: usize,
    #[serde(rename(serialize = "shelf with no elf on it"))]
    shelf_with_no_elf: usize,
}

#[debug_handler]
pub async fn count_elf(body: String) -> Json<CountElfResponse> {
    trace!("count_elf request body: {body}");

    let elf = body.matches("elf").count();

    let elf_on_shelf_search = b"elf on a shelf";
    let elf_on_shelf = body
        .as_bytes()
        .windows(elf_on_shelf_search.len())
        .filter(|el| el == elf_on_shelf_search)
        .count();
    let shelf_with_no_elf = body.matches("shelf").count() - elf_on_shelf;

    Json(CountElfResponse {
        elf,
        elf_on_shelf,
        shelf_with_no_elf,
    })
}
