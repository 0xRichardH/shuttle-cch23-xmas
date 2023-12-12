pub mod errors;
pub mod handlers;

use std::{collections::HashMap, fmt::Display};

use serde::{ser::SerializeStruct, Deserialize, Serialize, Serializer};

#[derive(Debug, Clone)]
pub enum ReindeerFood {
    Grass,
    Hay,
    Pizza,
    Unknown(String),
}

impl<'de> Deserialize<'de> for ReindeerFood {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?.to_lowercase();
        match s.as_str() {
            "grass" => Ok(ReindeerFood::Grass),
            "hay" => Ok(ReindeerFood::Hay),
            "pizza" => Ok(ReindeerFood::Pizza),
            _ => Ok(ReindeerFood::Unknown(s)),
        }
    }
}

impl Display for ReindeerFood {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReindeerFood::Grass => write!(f, "grass"),
            ReindeerFood::Hay => write!(f, "hay"),
            ReindeerFood::Pizza => write!(f, "pizza"),
            ReindeerFood::Unknown(s) => write!(f, "{}", s),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Reindeer {
    name: String,
    strength: u32,
    speed: Option<f32>,
    height: Option<u32>,
    antler_width: Option<u32>,
    snow_magic_power: Option<u32>,
    favorite_food: Option<ReindeerFood>,
    #[serde(rename = "cAnD13s_3ATeN-yesT3rdAy")]
    candy_eaten_yesterday: Option<u32>,
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

    pub fn antler_width(&self) -> Option<u32> {
        self.antler_width
    }

    pub fn snow_magic_power(&self) -> Option<u32> {
        self.snow_magic_power
    }

    pub fn favorite_food(&self) -> Option<ReindeerFood> {
        self.favorite_food.clone()
    }

    pub fn candy_eaten_yesterday(&self) -> Option<u32> {
        self.candy_eaten_yesterday
    }
}

pub struct ReindeerContestStats {
    fastest: Reindeer,
    tallest: Reindeer,
    magician: Reindeer,
    consumer: Reindeer,
}

impl ReindeerContestStats {
    pub fn new(
        fastest: Reindeer,
        tallest: Reindeer,
        magician: Reindeer,
        consumer: Reindeer,
    ) -> Self {
        Self {
            fastest,
            tallest,
            magician,
            consumer,
        }
    }
}

impl Serialize for ReindeerContestStats {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("ReindeerContestStats", 4)?;
        state.serialize_field(
            "fastest",
            format!(
                "Speeding past the finish line with a strength of {} is {}",
                self.fastest.strength(),
                self.fastest.name()
            )
            .as_str(),
        )?;

        if let Some(antler_width) = self.tallest.antler_width() {
            state.serialize_field(
                "tallest",
                format!(
                    "{} is standing tall with his {} cm wide antlers",
                    self.tallest.name(),
                    antler_width,
                )
                .as_str(),
            )?;
        }

        if let Some(snow_magic_power) = self.magician.snow_magic_power() {
            state.serialize_field(
                "magician",
                format!(
                    "{} could blast you away with a snow magic power of {}",
                    self.magician.name(),
                    snow_magic_power,
                )
                .as_str(),
            )?;
        }

        if let Some(favorite_food) = self.consumer.favorite_food() {
            state.serialize_field(
                "consumer",
                format!(
                    "{} ate lots of candies, but also some {}",
                    self.consumer.name(),
                    favorite_food
                )
                .as_str(),
            )?;
        }

        state.end()
    }
}

#[derive(Debug, Deserialize, Default)]
pub struct CookieIngredient {
    flour: u32,
    sugar: u32,
    butter: u32,
    #[serde(rename = "baking powder")]
    baking_powder: u32,
    #[serde(rename = "chocolate chips")]
    chocolate_chips: u32,
}

impl CookieIngredient {
    pub fn from(r: &HashMap<String, u32>) -> Option<Self> {
        let ingredient = Self {
            flour: r.get("flour")?.to_owned(),
            sugar: r.get("sugar")?.to_owned(),
            butter: r.get("butter")?.to_owned(),
            baking_powder: r.get("baking powder")?.to_owned(),
            chocolate_chips: r.get("chocolate chips")?.to_owned(),
        };

        Some(ingredient)
    }
}

#[derive(Debug, Deserialize)]
pub struct Pokemon {
    id: u64,
    name: String,
    base_experience: u32,
    height: u32,
    is_default: bool,
    order: u32,
    weight: u32,
}
