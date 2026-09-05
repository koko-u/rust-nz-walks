use crate::features::regions::models as r_models;
use crate::features::walks::models;
use uom::si::length;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct WalkDto {
    pub id: models::WalkId,
    pub name: String,
    pub description: Option<String>,
    pub length: Option<LengthKm>,
    pub image_url: Option<String>,
    pub region: r_models::Region,
    pub difficulty: Option<models::Difficulty>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct LengthKm {
    pub value: f64,
    pub unit: String,
}

impl From<models::Walk> for WalkDto {
    fn from(walk: models::Walk) -> Self {
        Self {
            id: walk.id,
            name: walk.name,
            description: walk.description,
            length: walk.length.map(LengthKm::from),
            image_url: walk.image_url,
            region: walk.region,
            difficulty: walk.difficulty,
        }
    }
}

impl From<uom::si::f64::Length> for LengthKm {
    fn from(length: uom::si::f64::Length) -> Self {
        Self {
            value: length.get::<length::kilometer>(),
            unit: "km".to_string(),
        }
    }
}