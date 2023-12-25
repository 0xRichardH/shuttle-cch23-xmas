use crate::{app_state::AppState, prelude::*, utils::RequestClient};
use axum::extract::{Path, State};
use s2::{cellid::CellID, latlng::LatLng};

pub async fn parse_coords(Path(cell_id): Path<String>) -> Result<String> {
    let lat_lng = parse_coordinates_from_s2_cell_id(&cell_id)?;
    let lat = decimal_to_dms_lat(lat_lng.lat.deg());
    let lng = decimal_to_dms_lng(lat_lng.lng.deg());
    Ok(f!("{lat} {lng}"))
}

pub async fn get_country_from_coords(
    State(state): State<AppState>,
    Path(cell_id): Path<String>,
) -> Result<String> {
    let lat_lng = parse_coordinates_from_s2_cell_id(&cell_id)?;

    let Some(api_key) = state.secrect_store.get("MAPSCO_API_KEY") else {
        return Err(AppError::Internal(anyhow::anyhow!(
            "Failed to get MAPSCO_API_KEY from secret store"
        )));
    };
    let country = get_country_from_lat_lng(lat_lng, &api_key).await?;
    tracing::info!("get country: {country:?}");
    Ok(country)
}

fn parse_coordinates_from_s2_cell_id(cell_id: &str) -> Result<LatLng> {
    let cell_id = u64::from_str_radix(cell_id, 2)?;
    let hex_cell_id = f!("{:x}", cell_id);
    let cell = CellID::from_token(&hex_cell_id);
    let lat_lng = LatLng::from(cell);
    Ok(lat_lng)
}

fn decimal_to_dms_lat(deg: f64) -> String {
    let direction = if deg >= 0.0 { "N" } else { "S" };
    decimal_to_dms(deg, direction)
}

fn decimal_to_dms_lng(deg: f64) -> String {
    let direction = if deg >= 0.0 { "E" } else { "W" };
    decimal_to_dms(deg, direction)
}

fn decimal_to_dms(deg: f64, direction: &str) -> String {
    // Convert to absolute value
    let deg = deg.abs();

    // Degrees, minutes, and seconds
    let d = deg.floor() as i32;
    let m = ((deg - d as f64) * 60.0).floor() as i32;
    let s = (deg - d as f64 - m as f64 / 60.0) * 3600.0;

    // Format the string
    f!("{:02}°{:02}'{:.3}''{}", d, m, s, direction)
}

async fn get_country_from_lat_lng(lat_lng: LatLng, api_key: &str) -> Result<String> {
    let url = f!(
        "https://geocode.maps.co/reverse?lat={lat}&lon={lng}&api_key={api_key}",
        lat = lat_lng.lat.deg(),
        lng = lat_lng.lng.deg()
    );

    let body = RequestClient::new(3).get(&url).await?.text().await?;
    tracing::info!("get country from lat lng: {body:?}");
    let result = serde_json::from_str::<serde_json::Value>(&body)?;

    if let Some(country) = result.get("address").and_then(|a| a.get("country")) {
        let Some(country) = country.as_str() else {
            return Err(AppError::Internal(anyhow::anyhow!(
                "Failed to convert country to string"
            )));
        };
        Ok(country.to_string())
    } else {
        Err(AppError::Internal(anyhow::anyhow!(
            "Failed to get country from lat lng"
        )))
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parse_coordinates_from_s2_cell_id() {
        let cell_id = "0100111110010011000110011001010101011111000010100011110001011011";
        assert_eq!(
            parse_coordinates_from_s2_cell_id(cell_id).unwrap(),
            LatLng::from_degrees(83.66508998386551, -30.627939871985497)
        );
    }

    #[test]
    fn test_decimal_to_dms_lat() {
        assert_eq!(decimal_to_dms_lat(83.66508998386551), "83°39'54.324''N");
    }

    #[test]
    fn test_decimal_to_dms_lng() {
        assert_eq!(decimal_to_dms_lng(-30.627939871985497), "30°37'40.584''W");
    }
}
