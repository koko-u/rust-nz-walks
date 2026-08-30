use uom::si;

use crate::features::regions::models as r_models;
use crate::features::walks::models;
use crate::features::walks::rows;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Walk {
    pub id: models::WalkId,
    pub name: String,
    pub description: Option<String>,
    pub length: Option<si::f64::Length>,
    pub image_url: Option<String>,
    pub region: r_models::Region,
    pub difficulty: Option<models::Difficulty>,
}

impl From<rows::WalkRow> for Walk {
    fn from(row: rows::WalkRow) -> Self {
        Self {
            id: row.id.into(),
            name: row.name,
            description: row.description,
            length: row.length_km.map(si::f64::Length::new::<si::length::kilometer>),
            image_url: row.image_url,
            region: r_models::Region {
                id: row.region_id.into(),
                code: row.region_code,
                name: row.region_name,
                image_url: row.region_image_url,
            },
            difficulty: row.difficulty,
        }
    }
}
