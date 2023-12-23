use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Order {
    pub id: i32,
    pub region_id: i32,
    pub gift_name: String,
    pub quantity: i32,
}

#[derive(Debug, Deserialize)]
pub struct Region {
    pub id: i32,
    pub name: String,
}
