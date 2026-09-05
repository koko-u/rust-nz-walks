use crate::features::walks::models;

#[derive(Debug, Clone, PartialEq)]
pub struct CreateCommand {
    pub name: String,
    pub description: Option<String>,
    pub length: Option<f64>,
    pub image_url: Option<String>,
    pub region_code: String,
    pub difficulty: Option<models::Difficulty>,
}