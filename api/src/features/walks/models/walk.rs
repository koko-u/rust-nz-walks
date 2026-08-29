use uom::si;

use super::Difficulty;
use super::Region;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Walk {
    pub id: uuid::Uuid,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub length: Option<si::f64::Length>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    pub region: Region,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub difficulty: Option<Difficulty>,
}
