use std::io::Cursor;

use axum::body;
use tar::{Archive, Entries};

use crate::prelude::*;

pub async fn count_archive_files(bytes: body::Bytes) -> Result<String> {
    let mut a = Archive::new(Cursor::new(bytes));
    let counter = a
        .entries()
        .map_err(|e| anyhow::anyhow!("failed to count files: {e}"))?
        .count();
    Ok(counter.to_string())
}

pub async fn count_archive_files_size(bytes: body::Bytes) -> Result<String> {
    let mut a = Archive::new(Cursor::new(bytes));
    let entries = a
        .entries()
        .map_err(|e| anyhow::anyhow!("failed to count files: {e}"))?;
    let mut size = 0;
    for file in entries {
        size += file?.size();
    }

    Ok(size.to_string())
}
