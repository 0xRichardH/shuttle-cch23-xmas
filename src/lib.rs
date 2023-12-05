pub mod handlers;

use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub enum ReindeerFood {
    #[serde(rename_all = "lowercase")]
    Grass,
    Hay,
}

impl Copy for ReindeerFood {}

#[derive(Deserialize)]
pub struct Reindeer {
    name: String,
    strength: u32,
    speed: Option<f32>,
    height: Option<u32>,
    antler_width: Option<u32>,
    snow_magic_power: Option<u32>,
    favorite_food: Option<ReindeerFood>,
}

impl Reindeer {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn strength(&self) -> u32 {
        self.strength
    }

    pub fn speed(&self) -> Option<f32> {
        self.speed
    }

    pub fn height(&self) -> Option<u32> {
        self.height
    }

    pub fn snow_magic_power(&self) -> Option<u32> {
        self.snow_magic_power
    }

    pub fn favorite_food(&self) -> Option<ReindeerFood> {
        self.favorite_food
    }
}
