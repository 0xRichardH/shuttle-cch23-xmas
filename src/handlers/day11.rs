use axum_extra::extract::Multipart;

use crate::errors::AppError;

pub async fn red_pixels(mut multipart: Multipart) -> anyhow::Result<String, AppError> {
    let mut red_counter = 0;
    if let Some(field) = multipart.next_field().await? {
        let data = field.bytes().await?;
        let image = image::load_from_memory(&data)?.to_rgba8();
        for pix in image.pixels() {
            let (red, green, blue) = (pix[0], pix[1], pix[2]);
            if let Some(gb) = green.checked_add(blue) {
                if red > gb {
                    red_counter += 1;
                }
            }
        }
        tracing::debug!("red_counter = {}", red_counter);
    }

    Ok(red_counter.to_string())
}
