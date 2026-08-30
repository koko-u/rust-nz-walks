use crate::features::walks::models::Difficulty;

pub struct WalkRow {
    pub id: uuid::Uuid,
    pub name: String,
    pub description: Option<String>,
    pub length_km: Option<f64>,
    pub image_url: Option<String>,
    pub region_id: uuid::Uuid,
    pub region_code: String,
    pub region_name: String,
    pub region_image_url: Option<String>,
    pub difficulty: Option<Difficulty>,
}